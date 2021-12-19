use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;
use std::collections::BTreeSet;

type Scanner = BTreeSet<(i32, i32, i32)>;

fn main() {
    let mut scanners = vec![];
    let mut scanner_num = 0;
    let mut oriented_scanners_and_offsets = vec![];
    let mut final_board;

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

    println!("Looking over all orientations...");

    final_board = scanners[0].clone();
    for _ in 1..=scanners.len() {
        search(&mut final_board, &mut oriented_scanners_and_offsets, &scanners);
    }

    // Find all overlaps of more than 12 among the first scanner set and the rest.
    // Keep track of any that still don't overlap
    // Test of those overlap with the larger set of points.

    for pt in &final_board {
        println!("{:?}", pt);
    }
    println!("Total beacons: {}", final_board.iter().count());
}

fn search(final_board: &mut Scanner, oriented_scanners_and_offsets: &mut Vec<(Scanner, (i32, i32, i32))>, scanners: &Vec<Scanner>) {
    for s_idx in 1..scanners.len() {
        let oriented_scanners = all_orientations(&scanners[s_idx]);
        for s in oriented_scanners {
            match find_overlap(&final_board, &s) {
                Some((offset_scanner, offset)) => {
                    *final_board = final_board.union(&offset_scanner).cloned().collect();
                    oriented_scanners_and_offsets.push((offset_scanner, offset));
                    break;
                },
                None => ()
            }
        }
    }
}

// Generate all 24 orientations of a scanner
fn all_orientations(scanner: &Scanner) -> Vec<Scanner> {
    // Rotation is like: 2, 1 -> -1, 2 -> -2, -1 -> 1, -2
    let mut oriented_scanners: Vec<Scanner> = vec![];

    oriented_scanners.push(scanner.clone());

    // Rotate y/z along +/- x
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, y, z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, -z, y)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, -y, -z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, z, -y)).collect());

    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (x, -z, y)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (x, -y, -z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (x, z, -y)).collect());

    // Rotate x/y along +/- z
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (x, -y, z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-z, -y, x)).collect());
    
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (z, -y, -x)).collect());

    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-z, y, x)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, y, -z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (z, y, -x)).collect());

    // Rotate x/z along +/- y
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (x, y, -z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-y, x, -z)).collect());
    
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (y, -x, -z)).collect());

    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-y, x, z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (-x, -y, z)).collect());
    oriented_scanners.push(scanner.iter().map(|&(x, y, z)| (y, -x, z)).collect());

    return oriented_scanners;
}

fn find_overlap(target: &Scanner, candidate: &Scanner) -> Option<(Scanner, (i32, i32, i32))> {
    let anchor_pt = target.iter().next().unwrap();
    let (a_x, a_y, a_z) = anchor_pt;

    for candidate_anchor in candidate.iter() {
        let (c_x, c_y, c_z) = candidate_anchor;
        // print!(".");

        // For every beacon in the candidate Scanner's list, transform the list as if
        // that point were the anchor point, then check the overlap. If the overlap is 12,
        // then we have the matching orientation.
        let offset = (c_x - a_x, c_y - a_y, c_z - a_z);
        let (off_x, off_y, off_z) = offset;

        let offset_candidate: Scanner = candidate.iter().map(|&(x, y, z)| (x - off_x, y - off_y, z - off_z)).collect();

        let count = target.intersection(&offset_candidate).count();
        if count >= 12 {
            // println!("Found overlap");
            return Some((offset_candidate, offset));
        }
    }

    return None;
}
