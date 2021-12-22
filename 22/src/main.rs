use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;
use std::ops;


fn main() {
    let (expected_on, instructions) = load_instructions();

    // for i in instructions {
    //     println!("{:?}", i);
    // }

    let mut cube = Cube::new();
    for (s, i) in instructions.iter().enumerate() {
        // println!("Step {}", s);
        cube.run_step(&i);
    }

    if expected_on.is_some() {
        println!("Expect {} illuimnated in initialization region", expected_on.unwrap());
    }
    println!("Found  {} illuminated", cube.lights_on());
}

fn load_instructions() -> (Option<u32>, Vec<Instruction>) {
    let mut expected_on = None;
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                if let Some(_) = line_str.find("expect") {
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

    return (expected_on, instructions);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Lightstate {
    On,
    Off
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

struct Cube {
    cube: [[[Lightstate; SIZE]; SIZE]; SIZE]
}

impl Cube {
    fn new() -> Cube {
        Cube {
            cube: [[[Lightstate::Off; SIZE]; SIZE]; SIZE]
        }
    }

    fn set(&mut self, light_state: Lightstate, x: i32, y: i32, z: i32) {
        // println!("Turning {:?} ({},{},{})", light_state, x, y, z);
        let x: usize = (x - LOW) as usize;
        let y: usize = (y - LOW) as usize;
        let z: usize = (z - LOW) as usize;
        self.cube[x][y][z] = light_state;
    }

    fn run_step(&mut self, inst: &Instruction) {
        if !self.can_apply(inst) {
            // println!("Can't run this step");
            return;
        }

        for x in clamp_rg(inst.x_rg) {
            for y in clamp_rg(inst.y_rg) {
                for z in clamp_rg(inst.z_rg) {
                    self.set(inst.light_state, x, y, z);
                }
            }
        }
    }

    fn can_apply(&self, inst: &Instruction) -> bool {
        ranges_overlap(inst.x_rg, RANGE) &&
        ranges_overlap(inst.y_rg, RANGE) &&
        ranges_overlap(inst.z_rg, RANGE)
    }

    fn lights_on(&self) -> u32 {
        let mut count = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    count += match self.cube[x][y][z] {
                        Lightstate::On => 1,
                        Lightstate::Off => 0
                    }
                }
            }
        }
        return count;
    }
}

fn clamp_rg((lo, hi): (i32, i32)) -> ops::RangeInclusive<i32> {
    let lo = num::clamp(lo, LOW, HIGH);
    let hi = num::clamp(hi, LOW, HIGH);
    return lo..=hi;
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
