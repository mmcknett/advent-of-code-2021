use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;
use std::ops;
use std::cmp;
use std::collections::BTreeSet;


fn main() {
    let (expected_on, expect_on_p2, instructions) = load_instructions();

    // for i in instructions {
    //     println!("{:?}", i);
    // }

    let mut cube = Cube::new();
    for (_s, i) in instructions.iter().enumerate() {
        // println!("Step {}", s);
        cube.run_step(&i);
        // println!("Final state of Step {} lights:", s);
        // for vol in &cube.on_vols {
        //     println!("{:?}", &vol);
        // }
    }

    if expected_on.is_some() {
        println!("Expect {} illuimnated in initialization region", expected_on.unwrap());
    }
    println!("Found  {} illuminated", cube.lights_on());



    println!("Part 2...");

    let mut cube = Cube::new();
    cube.initialization_mode = false; // Part 2 is the same, but with a bigger range.

    for (_s, i) in instructions.iter().enumerate() {
        // println!("Step {}", s);
        cube.run_step(&i);
        // println!("Final state of Step {} lights:", s);
        // for vol in &cube.on_vols {
        //     println!("{:?}", &vol);
        // }
    }

    if expect_on_p2.is_some() {
        println!("Expect {} illuminated for full board", expect_on_p2.unwrap());
    }
    println!("Found  {} illuminated", cube.lights_on());
}

fn load_instructions() -> (Option<u32>, Option<u64>, Vec<Instruction>) {
    let mut expected_on = None;
    let mut expected_on_p2 = None;
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                if let Some(_) = line_str.find("expect2") {
                    let re = Regex::new(r" (\d+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    expected_on_p2 = Some(u64::from_str(&cap[1]).unwrap());
                }
                else if let Some(_) = line_str.find("expect") {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    expected_on = Some(u32::from_str(&cap[1]).unwrap());
                }
                else if line_str != "" {
                    let re = Regex::new(r"(on|off) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    let inst = Instruction {
                        light_state: Lightstate::from_str(&cap[1]).unwrap(),
                        x_rg: normalize(i32::from_str(&cap[2]).unwrap(), i32::from_str(&cap[3]).unwrap()),
                        y_rg: normalize(i32::from_str(&cap[4]).unwrap(), i32::from_str(&cap[5]).unwrap()),
                        z_rg: normalize(i32::from_str(&cap[6]).unwrap(), i32::from_str(&cap[7]).unwrap())
                    };
                    instructions.push(inst);
                }
            },
            Err(_) => ()
        }
    }

    return (expected_on, expected_on_p2, instructions);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Lightstate {
    On,
    Off
}

impl Lightstate {
    fn is_on(&self) -> bool {
        match self {
            Lightstate::On => true,
            Lightstate::Off => false
        }
    }
}

impl FromStr for Lightstate {
    type Err = ();

    fn from_str(input: &str) -> Result<Lightstate, Self::Err> {
        match input {
            "on"  => Ok(Lightstate::On),
            "off"  => Ok(Lightstate::Off),
            _      => Err(()),
        }
    }
}

fn normalize(a1: i32, a2: i32) -> (i32, i32) {
    if a1 < a2 { (a1, a2) } else { (a2, a1) }
}

#[derive(Debug)]
struct Instruction {
    light_state: Lightstate,
    x_rg: (i32, i32),
    y_rg: (i32, i32),
    z_rg: (i32, i32)
}

const LOW: i32 = -50;
const HIGH: i32 = 50;
const RANGE: (i32, i32) = (LOW, HIGH);
const SIZE: usize = (HIGH - LOW + 1) as usize;

#[derive(Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }
    fn from(coord: (i32, i32, i32)) -> Coord {
        let (x, y, z) = coord;
        Coord::new(x, y, z)
    }
}

struct Cube {
    on_vols: BTreeSet<LightVol>,
    initialization_mode: bool
}

impl Cube {
    fn new() -> Cube {
        Cube {
            on_vols: BTreeSet::new(),
            initialization_mode: true
        }
    }

    fn run_step(&mut self, inst: &Instruction) {
        if !self.can_apply(inst) {
            // println!("Can't run this step");
            return;
        }

        let inst_volume = match self.initialization_mode {
            true => LightVol::ranges(clamped_rg(inst.x_rg), clamped_rg(inst.y_rg), clamped_rg(inst.z_rg)),
            false => inst.make_lightvol()
        };
        // println!("Working with volume {:?}", inst_volume);

        // Weird algorithm to turn on the lights:
        // 1. turn them off
        // 2. if this is an "on" instruction, make a new volume for the on lights.

        let overlaps: Vec<LightVol> = self.on_vols.iter().filter(|vol| volumes_overlap(&inst_volume, *vol)).cloned().collect();
        for vol in overlaps {
            // println!("Removing overlapping range: {:?}", vol);
            self.on_vols.remove(&vol);
            let new_vols = split_volume(vol, &inst_volume);
            for new_vol in new_vols {
                // println!("Adding range: {:?}", new_vol);
                self.on_vols.insert(new_vol);
            }
        }

        if inst.light_state == Lightstate::On {
            // println!("Turning on {:?}", inst_volume);
            self.on_vols.insert(inst_volume);
        }
    }

    fn can_apply(&self, inst: &Instruction) -> bool {
        match self.initialization_mode {
            true => volumes_overlap(&inst.make_lightvol(), &LightVol::ranges(RANGE, RANGE, RANGE)),
            false => true
        }
    }

    fn lights_on(&self) -> u64 {
        self.on_vols.iter().fold(0, |sum, vol| sum + vol.count())
    }
}

fn clamp_rg(rg: (i32, i32)) -> ops::RangeInclusive<i32> {
    let (lo, hi) = clamped_rg(rg);
    return lo..=hi;
}

fn clamped_rg((lo, hi): (i32, i32)) -> (i32, i32) {
    let lo = num::clamp(lo, LOW, HIGH);
    let hi = num::clamp(hi, LOW, HIGH);
    return (lo, hi);
}

// Ranges are assumed to be (low, high)
//
//  |------------|
// |--------------|
//
//  |------|
//   |---|
//
//     |-------------|
// |------|
//
//    |-----|
// |-|
fn ranges_overlap(rg_a: (i32, i32), rg_b: (i32, i32)) -> bool {
    let (a_l, a_h) = rg_a;
    let (b_l, b_h) = rg_b;
    // Ranges overlap if the start of b comes before the end of a AND
    // the end of b comes after the start of a.
    return b_l <= a_h && b_h >= a_l;
}

fn range_size_incl(rg: (i32, i32)) -> u64 {
    (rg.1 - rg.0 + 1) as u64
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
struct LightVol {
    x_rg: (i32, i32),
    y_rg: (i32, i32),
    z_rg: (i32, i32)
}

impl LightVol {
    fn new(x_l: i32, x_h: i32, y_l: i32, y_h: i32, z_l: i32, z_h: i32) -> LightVol {
        LightVol { x_rg: (x_l, x_h), y_rg: (y_l, y_h), z_rg: (z_l, z_h) }
    }

    fn ranges(x_rg: (i32, i32), y_rg: (i32, i32), z_rg: (i32, i32)) -> LightVol {
        LightVol { x_rg, y_rg, z_rg }
    }

    fn count(&self) -> u64 {
        range_size_incl(self.x_rg) * range_size_incl(self.y_rg) * range_size_incl(self.z_rg)
    }
}

impl Instruction {
    fn make_lightvol(&self) -> LightVol {
        LightVol {
            x_rg: self.x_rg,
            y_rg: self.y_rg,
            z_rg: self.z_rg
        }
    }
}

fn volumes_overlap(vol_a: &LightVol, vol_b: &LightVol) -> bool {
    ranges_overlap(vol_a.x_rg, vol_b.x_rg) &&
    ranges_overlap(vol_a.y_rg, vol_b.y_rg) &&
    ranges_overlap(vol_a.z_rg, vol_b.z_rg)
}

// Splits on_vol into volumes baed on off_vol, and returns only volumes
// where the lights are on afterward.
fn split_volume(on_vol: LightVol, off_vol: &LightVol) -> Vec<LightVol> {
    if !volumes_overlap(&on_vol, &off_vol) {
        return vec![on_vol];
    }

    let mut volumes = vec![];

    let x_rgs = split_light_range(on_vol.x_rg, off_vol.x_rg);
    let y_rgs = split_light_range(on_vol.y_rg, off_vol.y_rg);
    let z_rgs = split_light_range(on_vol.z_rg, off_vol.z_rg);

    for &(ls_x, x_rg) in &x_rgs {
        for &(ls_y, y_rg) in &y_rgs {
            for &(ls_z, z_rg) in &z_rgs {
                if ls_x.is_on() || ls_y.is_on() || ls_z.is_on() {
                    volumes.push(LightVol::ranges(x_rg, y_rg, z_rg));
                }
            }
        }
    }

    return volumes;
}

fn split_light_range(on_rg: (i32, i32), off_rg: (i32, i32)) -> Vec<(Lightstate, (i32, i32))> {
    if !ranges_overlap(on_rg, off_rg) {
        panic!("Can' split non-overlapping ranges.");
    }

    let mut ranges = vec![];
    let (on_l, on_h) = on_rg;
    let (off_l, off_h) = off_rg;

    if on_l < off_l {
        ranges.push((Lightstate::On, (on_l, off_l - 1)));
    }

    ranges.push((Lightstate::Off, (cmp::max(off_l, on_l), cmp::min(on_h, off_h))));

    if off_h < on_h {
        ranges.push((Lightstate::On, (off_h + 1, on_h)));
    }

    return ranges;
}

#[cfg(test)]
mod light_range_tests {
    use super::{split_volume, split_light_range, LightVol};
    use std::collections::BTreeSet;
    use super::Lightstate::*;

    #[test]
    fn split_range() {
        assert_eq!(vec![(On, (0, 0)), (Off, (1, 2))], split_light_range((0, 2), (1, 3)));
        assert_eq!(vec![(Off, (0, 2))], split_light_range((0, 2), (0, 3)));
        assert_eq!(vec![(Off, (0, 3))], split_light_range((0, 3), (0, 3)));
        assert_eq!(vec![(Off, (0, 3)), (On, (4, 4))], split_light_range((0, 4), (0, 3)));
        assert_eq!(vec![(Off, (0, 2))], split_light_range((0, 2), (-1, 3)));
        assert_eq!(vec![(On, (0, 0)), (Off, (1, 3)), (On, (4, 4))], split_light_range((0, 4), (1, 3)));
        assert_eq!(vec![(On, (0, 1)), (Off, (2, 3)), (On, (4, 4))], split_light_range((0, 4), (2, 3)));
        assert_eq!(vec![(On, (0, 1)), (Off, (2, 2)), (On, (3, 4))], split_light_range((0, 4), (2, 2)));
        assert_eq!(vec![(On, (0, 0)), (Off, (1, 4))], split_light_range((0, 4), (1, 4)));
        assert_eq!(vec![(On, (0, 0)), (Off, (1, 4))], split_light_range((0, 4), (1, 5)));
        assert_eq!(vec![(Off, (0, 2)), (On, (3, 4))], split_light_range((0, 4), (0, 2)));
    }

    #[test]
    fn splits_cube_correctly() {
        let on_vol = LightVol {
            x_rg: (0, 2),
            y_rg: (0, 2), 
            z_rg: (0, 2)
        };

        let off_vol = LightVol {
            x_rg: (1, 2),
            y_rg: (1, 2),
            z_rg: (1, 2)
        };

        let expected: BTreeSet<LightVol> = [
            LightVol::new(0, 0, 0, 0, 0, 0),
            LightVol::new(1, 2, 0, 0, 0, 0),
            LightVol::new(0, 0, 1, 2, 0, 0),
            LightVol::new(0, 0, 0, 0, 1, 2),
            LightVol::new(1, 2, 1, 2, 0, 0),
            LightVol::new(0, 0, 1, 2, 1, 2),
            LightVol::new(1, 2, 0, 0, 1, 2)
        ].into_iter().collect();

        let result: BTreeSet<LightVol> = split_volume(on_vol, &off_vol).into_iter().collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn splits_bar_correctly() {
        let on_vol = LightVol::new(0, 2, 0, 2, 0, 2);
        let off_vol = LightVol::new(0, 2, 0, 2, 1, 2);

        let expected = LightVol::new(0, 2, 0, 2, 0, 0);

        let result = split_volume(on_vol, &off_vol);

        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn returns_on_if_no_overlap() {
        let on_vol = LightVol::new(0, 2, 0, 1, 0, 2);
        let off_vol = LightVol::new(0, 2, 2, 3, 0, 2);
        
        let result = split_volume(on_vol, &off_vol);

        assert_eq!(1, result.len());
        assert_eq!(on_vol, result[0]);
    }
}
