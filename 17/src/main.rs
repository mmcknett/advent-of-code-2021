use std::ops;
use std::cmp::{self, Ordering};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Rect {
    l: i32,
    t: i32,
    r: i32,
    b: i32
}

type Vel = Coord;

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y }
    }

    fn from(tuple: (i32, i32)) -> Coord {
        Coord { x: tuple.0, y: tuple.1 }
    }
}

impl ops::Add<Vel> for Coord {
    type Output = Self;

    fn add(self, _rhs: Vel) -> Self {
        Self::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Rect {
    fn new(tl: Coord, br: Coord) -> Rect {
        Rect { l: tl.x, t: tl.y, r: br.x, b: br.y }
    }

    fn from(tl: (i32, i32), br: (i32, i32)) -> Rect {
        Rect::new(Coord::from(tl), Coord::from(br))
    }

    fn contains(&self, pt: &Coord) -> bool {
        pt.x >= self.l && pt.x <= self.r &&
        pt.y <= self.t && pt.y >= self.b
    }

    fn above(&self, pt: &Coord) -> bool {
        pt.y < self.b
    }

    fn leftof(&self, pt: &Coord) -> bool {
        pt.x > self.r
    }
    
    fn rightof(&self, pt: &Coord) -> bool {
        pt.x < self.l
    }
}

struct Simresult {
    last: Coord,
    max_height: i32
}

fn main() {
    let sample_rec = Rect::from((20, -5), (30, -10));
    let input_rec = Rect::from((79, -117), (137, -176));

    // let rec = &sample_rec;
    let rec = &input_rec;
    let mut max_height = 0;
    let mut initial_v = Vel {x: rec.l, y: rec.b}; // Point at bottom left of target
    // let mut initial_v = Vel {x: 7, y: 9}; // Point at bottom left of target
    let mut initial_v_of_max_height = initial_v.clone();

    // 3741 is too low.

    #[derive(PartialEq)]
    enum Moving {
        Left,
        Right
    }
    let mut dir = Moving::Right;
    let mut velocities = 0;

    // for run in 1..=10000 {
    //     let result = simulate(initial_v, rec);
    //     let last = result.last;

    //     // println!("Sim {} starting {:?} landed at {:?} from height {}", run, initial_v, last, result.max_height);

    //     if rec.contains(&last) {
    //         // println!("Intersected!");
    //         if result.max_height >= max_height {
    //             max_height = result.max_height;
    //             initial_v_of_max_height = initial_v.clone();
    //         }

    //         velocities += 1;

    //         initial_v.x += if dir == Moving::Left { -1 } else { 1 };
    //     } else if rec.leftof(&last) {
    //         // println!("Landed right");
    //         initial_v.x -= 1;
    //         initial_v.y += 1;
    //         dir = Moving::Left;
    //     } else if rec.rightof(&last) {
    //         // println!("Landed left");
    //         initial_v.x += 1;
    //         initial_v.y += 1;
    //         dir = Moving::Right;
    //     } else if rec.above(&last) {
    //         // println!("Landed below");
    //         initial_v.y += 1;
    //     } else {
    //         println!("....landed somewhere else??");
    //     }
    // }

    for y in -176..=1000 {
        for x in 1..=500 {
            let initial_v = Vel { x, y };
            let result = simulate(initial_v, rec);
            let last = result.last;

            if rec.contains(&last) {
                // println!("Intersected!");
                if result.max_height > max_height {
                    max_height = result.max_height;
                    initial_v_of_max_height = initial_v.clone();
                }
                velocities += 1;
            }
        }
    }

    println!("Max height achieved: {} using velocity {:?}", max_height, initial_v_of_max_height);
    println!("Found a total of {} intersecting initial velocities", velocities);
}

// Returns the coordinate where the projectile lands after it's below the target range
fn simulate(initial_v: Vel, target: &Rect) -> Simresult {
    let mut pos = Coord::new(0, 0);
    let mut vel = initial_v;
    let mut max_height = 0;
    // println!("start: {:?}", pos);
    loop {
        pos = pos + vel;
        max_height = cmp::max(pos.y, max_height);

        vel.x = match vel.x.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Greater => vel.x - 1,
            Ordering::Less => vel.x + 1
        };
        vel.y -= 1;

        // println!("{:?}, {:?}", pos, target);

        if target.above(&pos) || target.contains(&pos) {
            return Simresult {
                max_height,
                last: pos
            };
        }
    }
}
