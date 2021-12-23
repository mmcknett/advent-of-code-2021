use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let hallway = load_hallway();
    println!("Hallway: {:?}", hallway);

    
}

fn load_hallway() -> Hallway {
    let mut hallway = Hallway::new();

    for line in io::stdin().lock().lines() {
        match line {
            Err(_) => (),
            Ok(line_str) => {
                let re = Regex::new(r"#(\.+)#").unwrap();
                if let Some(cap) = re.captures_iter(&line_str).next() {
                    hallway.hall = cap[1].chars().collect();
                }

                let re = Regex::new(r".(.#([A-D]#){4})").unwrap();
                if let Some(cap) = re.captures_iter(&line_str).next() {
                    // Load each amphipod into a room, and number the entryways
                    let mut next_entry: usize = 0;
                    for c in cap[1].chars() {
                        if ['A', 'B', 'C', 'D'].contains(&c) {
                            hallway.rooms[next_entry].push(c);
                            next_entry += 1;
                        }
                    }
                };
            }
        }
    }

    return hallway;
}


const NUM_ROOMS: usize = 4;
const ENTRYWAYS: [usize; NUM_ROOMS] = [2, 4, 6, 8];

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hallway {
    hall: Vec<char>,
    rooms: [Vec<char>; NUM_ROOMS], // four rooms with two positions,
    entryways: [usize; NUM_ROOMS]  // four doors to each room, at an offset in the hallway.
}

impl Hallway {
    fn new() -> Hallway {
        Hallway {
            hall: vec![],
            rooms: [vec![], vec![], vec![], vec![]],
            entryways: ENTRYWAYS
        }
    }
}