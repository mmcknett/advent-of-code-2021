use std::io::{self, BufRead};
use regex::Regex;
use std::cmp;
use std::fmt;
use std::iter;

fn main() {
    let mut hallway = load_hallway();

    // Debugging, set a hallway state.
    // hallway.hall[5] = 'B';
    // hallway.rooms[0].remove(0);
    // hallway.hall[7] = 'C';
    // hallway.rooms[1].remove(0);
    // hallway.hall[3] = 'D';
    // hallway.rooms[1].remove(0);

    println!("Hallway: {}", hallway);

    let mut lowest_energy_so_far = u32::MAX;
    let min_energy = find_min_energy(&hallway, 0, &mut lowest_energy_so_far);
    println!("Lowest organization energy: {}", min_energy);
}

fn find_min_energy(hallway: &Hallway, cost_so_far: u32, lowest_energy_so_far: &mut u32) -> u32 {
    // The minimum energy for a solved board is 0.
    if hallway.is_final() {
        *lowest_energy_so_far = cmp::min(*lowest_energy_so_far, cost_so_far);
        // println!("Lowest so far is {}", *lowest_energy_so_far);

        return *lowest_energy_so_far;
    }

    // If there isn't a final condition on any branch, we'll default to MAX.
    let mut min_energy = u32::MAX;
    let mut min_energy_state = None;
    let mut min_energy_cost = u32::MAX;

    let mut next_possible_states = vec![];

    for pos in 0..hallway.hall.len() {
        if let Some(cost_and_state) = hallway.next_state_from_hallway(pos) {
            next_possible_states.push(cost_and_state);
        }
    }

    for room in 0..NUM_ROOMS {
        let mut next_states = hallway.next_state_from_room(room);
        next_possible_states.append(&mut next_states);
    }

    for (cost, next_state) in next_possible_states {
        let cost_so_far_next = cost + cost_so_far;

        // Ignore any branches that top the lowest energy already found.
        if cost_so_far_next > *lowest_energy_so_far {
            continue;
        }

        let min_energy_next = find_min_energy(&next_state, cost_so_far_next, lowest_energy_so_far);
        if min_energy_next != u32::MAX && min_energy_next + cost < min_energy {
            min_energy = min_energy_next;

            // Bookkeeping
            min_energy_cost = cost;
            min_energy_state = Some(next_state)
        }
    }

    // println!("Min energy: {}", min_energy);
    // println!("Cost: {}, State: {}", min_energy_cost, if let Some(s) = min_energy_state { s.to_string() } else { String::from("") });

    return min_energy;
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
const ROOM_SIZE: usize = 2;
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

    fn is_final(&self) -> bool {
        self.rooms[0] == ['A', 'A'] &&
        self.rooms[1] == ['B', 'B'] &&
        self.rooms[2] == ['C', 'C'] &&
        self.rooms[3] == ['D', 'D']
    }

    fn next_state_from_hallway(&self, pos: usize) -> Option<(u32, Hallway)> {
        let c = self.hall[pos];
        return match c {
            '.' => None,
            'A' | 'B' | 'C' | 'D' => {
                let room = Self::allowed_room(c);
                let entryway = self.entryways[room];
                let hallsteps = (self.entryways[room] as i32 - pos as i32).abs() as usize;
                let room_has_only_allowed_amphipods = self.rooms[room].iter().all(|&c| Self::allowed_room(c) == room);
                
                let start = cmp::min(entryway, pos + 1);
                let end = cmp::max(entryway, if pos == 0 { 0 } else { pos - 1 }); // just avoid the underflow and hope the entryway is never at 0.
                let not_blocked = self.hall.iter().skip(start).take(end - start).all(|&c| c == '.');

                // println!("only has allowed? {}; not blocked? {}", room_has_only_allowed_amphipods, not_blocked);

                if room_has_only_allowed_amphipods && not_blocked {
                    let mut next = self.clone();
                    next.hall[pos] = '.';
                    next.rooms[room].insert(0, c);

                    let roomsteps = (ROOM_SIZE - self.rooms[room].len()) as u32;
                    let energy = Self::cost(c, hallsteps as u32 + roomsteps);

                    Some((energy, next))
                } else {
                    None
                }
            },
            _ => panic!("Unknown amphipod '{}'", c)
        };
    }

    fn next_state_from_room(&self, room: usize) -> Vec<(u32, Hallway)> {
        // It can go into any hallway position that is not an entryway or
        // blocked.
        // It can't exit a room if it's in the right room and no wrong ones are in
        // the room with it.
        // Don't worry about room-to-room -- let amphipods go into the hallway,
        // then into their room.
        if self.rooms[room].is_empty() ||
           self.rooms[room].iter().all(|&c| Self::allowed_room(c) == room)
        {
            return vec![];
        }

        let entry_pos = self.entryways[room];
        let roomsteps = (ROOM_SIZE - self.rooms[room].len()) as u32 + 1;
        let mut next_list = vec![];

        // Remove the amphipod from the room and find a new place for it in the hallway.
        let mut next_hallway = self.clone();
        let c = next_hallway.rooms[room].remove(0);

        let mut push_copy_of_hallway_with_pos = |pos| {
            let mut next = next_hallway.clone();
            next.hall[pos] = c;

            let cost = Self::cost(c, roomsteps + (entry_pos as i32 - pos as i32).abs() as u32);
            // println!("Cost {} for entry {} to pos {}, roomsteps is {}", cost, entry_pos, pos, roomsteps);
            next_list.push((cost, next));
        };

        for i in (0..entry_pos).rev() {
            // We hit a blocking amphipod; stop looking this way.
            if self.hall[i] != '.' { break; }

            if !self.entryways.contains(&i) {
                push_copy_of_hallway_with_pos(i);
            }
        }

        for i in entry_pos..self.hall.len() {
            // We hit a blocking amphipod; stop looking this way.
            if self.hall[i] != '.' { break; }

            if !self.entryways.contains(&i) {
                push_copy_of_hallway_with_pos(i);
            }
        }

        return next_list;
    }

    fn cost(c: char, steps: u32) -> u32 {
        steps * match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Unknown amphipod '{}'", c)
        }
    }

    fn allowed_room(c: char) -> usize {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => panic!("Unknown amphipod '{}'", c)
        }
    }

    fn room_from_hall_pos(&self, entry: usize) -> Option<usize> {
        for i in 0..NUM_ROOMS {
            if self.entryways[i] == entry {
                return Some(i);
            }
        }

        return None;
    }
}

impl fmt::Display for Hallway {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n#{}#\n", iter::repeat('#').take(self.hall.len()).collect::<String>())?;
        write!(f, "#{}#\n", self.hall.iter().collect::<String>())?;
        for room_pos in 0..ROOM_SIZE {
            for i in 0..(self.hall.len() + 2) {
                let i_test: i32 = i as i32 - 1;
                if i_test >= 0 && self.entryways.contains(&(i_test as usize)) {
                    let room = &self.rooms[self.room_from_hall_pos(i_test as usize).unwrap()];

                    let offset = ROOM_SIZE - room.len();
                    if room_pos >= offset {
                        write!(f, "{}", room[room_pos - offset])?;
                    } else {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}
