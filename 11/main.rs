use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let mut octos = [[0; 10]; 10];
    load_octos(&mut octos);

    let mut flashes = 0;

    // Part 1
    for step in 1..=100 {
        let mut initial_flashes = vec![];
        // First, increase every energy level
        for y in 0..10 {
            for x in 0..10 {
                octos[x][y] += 1;
                if octos[x][y] > 9 {
                    initial_flashes.push((x, y));
                    flashes += 1;
                }
            }
        }

        // Count and propagate flashes (anything that becomes 10 is a flash)
        for octo in initial_flashes {
            flashes += propagate_flashes(octo, &mut octos);
        }

        // Last, clamp to 0.
        for y in 0..10 {
            for x in 0..10 {
                if octos[x][y] > 9 {
                    octos[x][y] = 0;
                }
            }
        }

        println!("\nAfter step {}:", step);
        print_octos(&octos);
    }

    print!("\nTotal flahes: {}\n", flashes);
}

fn load_octos(octos: &mut [[i8; 10]; 10]) {
    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, num_char) in line.unwrap().chars().into_iter().enumerate() {
            octos[x][y] = num_char.to_digit(10).unwrap() as i8;
        }
    }

    println!("Before any steps:");
    print_octos(octos);
}

fn print_octos(octos: &[[i8; 10]; 10]) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", octos[x][y]);
        }
        print!("\n");
    }
}

fn propagate_flashes((x, y): (usize, usize), mut octos: &mut [[i8; 10]; 10]) -> u64 {
    let mut flash_count = 0;
    let mut next_flashes = vec![];

    let min_x = if x > 0 { x - 1 } else { 0 };
    let min_y = if y > 0 { y - 1 } else { 0 };
    let max_x = cmp::min(9, x + 1);
    let max_y = cmp::min(9, y + 1);

    for y_next in min_y..=max_y {
        for x_next in min_x..=max_x {
            octos[x_next][y_next] += 1;
            if octos[x_next][y_next] == 10 {
                next_flashes.push((x_next, y_next));
                flash_count += 1;
            }
        }
    }

    for octo in next_flashes {
        flash_count += propagate_flashes(octo, &mut octos);
    }

    return flash_count;
}
