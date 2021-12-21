use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;
use std::collections::BTreeSet;

type Scanner = BTreeSet<(i32, i32, i32)>;
const MIN_OVERLAPS: usize = 4;

fn main() {
    let mut scanners = vec![];
    let mut scanner_num = 0;
    let mut oriented_scanners_and_offsets: Vec<(Scanner, (i32, i32, i32), (usize, usize))> = vec![];
    let mut final_board: Scanner;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                if let Some(_) = line_str.find("scanner") {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    scanner_num = usize::from_str(&cap[1]).unwrap();
                    scanners.push(Scanner::new());
                } else if line_str != "" {
                    let re = Regex::new(r"([-\d]+),([-\d]+),([-\d]+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    let coord = (
                        i32::from_str(&cap[1]).unwrap(),
                        i32::from_str(&cap[2]).unwrap(),
                        i32::from_str(&cap[3]).unwrap()
                    );
                    scanners[scanner_num].insert(coord);
                }
            },
            Err(_) => ()
        }
    }

    // println!("Looking over all orientations...");

    // final_board = scanners[0].clone();
    // for s_idx in 0..scanners.len() {
    //     let mut final_board_for_this_scanner = Scanner::new();
    //     search(s_idx, &scanners, &mut final_board_for_this_scanner, &mut oriented_scanners_and_offsets);

    //     println!("Total beacons shared with the overlap of scanner {}: {}", s_idx, final_board_for_this_scanner.iter().count());
    //     for finding in &oriented_scanners_and_offsets {
    //         let (offset_scanner, _, (_, i)) = finding;
    //         scanners[*i] = offset_scanner.clone();
    //         final_board = final_board.union(&scanners[*i]).cloned().collect();
    //     }
    // }


    // Starting at 0, find all scanners that overlap, and reset them to the proper orientation.
    // Then, for each one found, find the scanners that overlap them.
    // Go until all scanners are discovered.
    let mut remaining_scanners: BTreeSet<usize> = (1..scanners.len()).collect();
    let mut queue = vec![0];
    final_board = scanners[0].clone();

    while !remaining_scanners.is_empty() {
        let cur_idx = queue.pop().unwrap();

        println!("Searching from {}", cur_idx);

        let result = search_simplified(cur_idx, &scanners, &remaining_scanners);
        for (oriented_scanner, oriented_idx) in result {
            println!("Found and oriented {}", oriented_idx);

            // Adjust the scanners queue so that this oriented scanner is now used instead.
            scanners[oriented_idx] = oriented_scanner.clone();

            // Expand the board with the newly-found scanner's information.
            final_board = final_board.union(&oriented_scanner).cloned().collect();

            // This scanner's beacons have been found, and we can search from it at a later point.
            remaining_scanners.remove(&oriented_idx);
            if !queue.contains(&oriented_idx) {
                queue.push(oriented_idx);
            }
        }
    }


    // println!("Oriented scanners & offsets: {:?}", oriented_scanners_and_offsets);

    // Find all overlaps of more than 12 among the first scanner set and the rest.
    // Keep track of any that still don't overlap
    // Test of those overlap with the larger set of points.

    println!(".....");
    // for pt in &final_board {
    //     println!("{:?}", pt);
    // }

    // 1142 is too high.
    // 921 is too high.
    println!("Total beacons: {}", final_board.iter().count());
}

// Finds all other scanners that overlap with the provided scanner
fn search_simplified(
    s_idx: usize,
    scanners: &Vec<Scanner>,
    remaining_scanners: &BTreeSet<usize>
) -> Vec<(Scanner, usize)>
{
    let mut oriented_scanners_and_offsets: Vec<(Scanner, usize)> = vec![];
    let scanner_in_question = &scanners[s_idx];
    for i in 0..scanners.len() {
        if !remaining_scanners.contains(&i) {
            continue;
        }

        // Try orienting each scanner to the scanner in question & finding overlap.
        let oriented_scanners = all_orientations(&scanners[i]);
        for s in oriented_scanners {
            match find_overlap(scanner_in_question, &s) {
                Some((offset_scanner, offset)) => {
                    oriented_scanners_and_offsets.push((offset_scanner, i));
                },
                None => ()
            }
        }
    }
    return oriented_scanners_and_offsets;
}

// Finds all other scanners that overlap with the provided scanner
fn search(
    s_idx: usize,
    scanners: &Vec<Scanner>,
    final_board: &mut Scanner,
    oriented_scanners_and_offsets: &mut Vec<(Scanner, (i32, i32, i32), (usize, usize))>
) {
    let scanner_in_question = &scanners[s_idx];
    for i in (s_idx + 1)..scanners.len() {
        // if i == s_idx {
        //     continue;
        // }

        // println!("Searching scanners {} and {}", s_idx, i);

        // Try orienting each scanner to the scanner in question & finding overlap.
        let oriented_scanners = all_orientations(&scanners[i]);
        for (o_i, s) in oriented_scanners.iter().enumerate() {
            match find_overlap(scanner_in_question, &s) {
                Some((offset_scanner, offset)) => {
                    *final_board = final_board.union(&offset_scanner).cloned().collect();
                    oriented_scanners_and_offsets.push((offset_scanner, offset, (s_idx, i)));
                    println!("Scanner {} overlaps with scanner {} in orientation {} with offset {:?}", s_idx, i, o_i, offset);
                    // break;
                },
                None => ()
            }
        }
    }
}

// Generate all 24 orientations of a scanner
fn all_orientations(scanner: &Scanner) -> Vec<Scanner> {
    let mut oriented_scanners: Vec<Scanner> = vec![];
    let mut curr = scanner.clone();

    // The plan...
    // Rot 4* around +x,
    // Rot z,
    // Rot 4* around +y,
    // Rot z,
    // Rot 4* around -x,
    // Rot z,
    // Rot 4* around -y
    // Rot x,
    // Rot 4* around +z,
    // Rot x * 2,
    // Rot 4* around -z
 
    for _ in 0..4 {
        curr = rotx(&curr);
        oriented_scanners.push(curr.clone());
    }
    curr = rotz(&curr);
    for _ in 0..4 {
        curr = roty(&curr);
        oriented_scanners.push(curr.clone());
    }
    curr = rotz(&curr);
    for _ in 0..4 {
        curr = rotx(&curr);
        oriented_scanners.push(curr.clone());
    }
    curr = rotz(&curr);
    for _ in 0..4 {
        curr = roty(&curr);
        oriented_scanners.push(curr.clone());
    }
    curr = rotx(&curr);
    for _ in 0..4 {
        curr = rotz(&curr);
        oriented_scanners.push(curr.clone());
    }
    curr = rotx(&curr);
    curr = rotx(&curr);
    for _ in 0..4 {
        curr = rotz(&curr);
        oriented_scanners.push(curr.clone());
    }

    return oriented_scanners;
}

fn rotx(start: &Scanner) -> Scanner {
    start.iter().map(|&(x, y, z)| (x, -z, y)).collect()
}

fn roty(start: &Scanner) -> Scanner {
    start.iter().map(|&(x, y, z)| (-z, y, x)).collect()
}

fn rotz(start: &Scanner) -> Scanner {
    start.iter().map(|&(x, y, z)| (-y, x, z)).collect()
}

fn find_overlap(target: &Scanner, candidate: &Scanner) -> Option<(Scanner, (i32, i32, i32))> {
    // Grab the first point in the target scanner result as the "anchor" point.
    for anchor_pt in target {
        let (a_x, a_y, a_z) = anchor_pt;
        
        for candidate_anchor in candidate.iter() {
            let (c_x, c_y, c_z) = candidate_anchor;
            // print!(".");

            // For every beacon in the candidate Scanner's list, transform the list as if
            // that point were the anchor point, then check the overlap. If the overlap is 12,
            // then we have the matching orientation.
            let offset = (c_x - a_x, c_y - a_y, c_z - a_z);
            let (off_x, off_y, off_z) = offset;

            // println!("Anchor {:?}, Candidate {:?}, Offset {:?}", anchor_pt, candidate_anchor, offset);

            let offset_candidate: Scanner = candidate.iter().map(|&(x, y, z)| (x - off_x, y - off_y, z - off_z)).collect();

            let count = target.intersection(&offset_candidate).count();
            // if count > 1 {
            //     println!("Overlap count: {}", count);
            // }
            if count >= MIN_OVERLAPS {
                // println!("Found overlap");
                return Some((offset_candidate, offset));
            }
        }
    }

    

    return None;
}
