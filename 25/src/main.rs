use std::io::{self, BufRead};
use std::collections::HashSet;
use std::cmp;

type Herd = HashSet<(usize, usize)>;

fn main() {
    let mut east_herd = Herd::new();
    let mut south_herd = Herd::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in io::stdin().lock().lines().enumerate() {
        let linestr = line.unwrap();
        for (x, c) in linestr.chars().enumerate() {
            match c {
                '>' => {
                    east_herd.insert((x, y));
                },
                'v' => {
                    south_herd.insert((x, y));
                },
                _ => ()
            };
            width = cmp::max(width, x + 1);
            height = cmp::max(height, y + 1);
        }
    }

    println!("Initial state:");
    print(&south_herd, &east_herd, width, height);

    let mut i: u32 = 0;
    loop {
        i += 1;
        let mut next_east = Herd::new();
        let mut next_south = Herd::new();

        for e_curr in &east_herd {
            let (e_x, e_y) = e_curr;
            let next_x = if e_x + 1 >= width { 0 } else { e_x + 1 };
            let e_next = (next_x, *e_y);

            if !south_herd.contains(&e_next) && !east_herd.contains(&e_next) {
                next_east.insert(e_next);
            } else {
                next_east.insert(e_curr.clone());
            }
        }

        for s_curr in &south_herd {
            let (s_x, s_y) = s_curr;
            let next_y = if s_y + 1 >= height { 0 } else { s_y + 1 };
            let s_next = (*s_x, next_y);

            if !south_herd.contains(&s_next) && !next_east.contains(&s_next) {
                next_south.insert(s_next);
            } else {
                next_south.insert(s_curr.clone());
            }
        }

        // println!("\nAfter step {}", i);
        // print(&south_herd, &east_herd, width, height);

        if next_south == south_herd && next_east == east_herd {
            break;
        }

        east_herd = next_east;
        south_herd = next_south;
    }

    println!("Cucumbers stop moving on step {}", i);
}

fn print(south_herd: &Herd, east_herd: &Herd, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let coord = (x, y);
            let c = if south_herd.contains(&coord) { 'v' } else if east_herd.contains(&coord) { '>' } else { '.' };
            print!("{}", c);
        }
        println!("");
    }
}
