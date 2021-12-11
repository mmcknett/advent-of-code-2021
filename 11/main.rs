use std::io::{self, BufRead};
use std::cmp;

type Octogrid = [[i8; 10]; 10];

fn main() {
    let mut octos = [[0; 10]; 10];
    load_octos(&mut octos);

    let mut flashes = 0;

    // Part 1
    for step in 1..=10000 {
        // First, increment energy and propagate flashes.
        for y in 0..10 {
            for x in 0..10 {
                let new_flashes = propagate_flashes((x, y), &mut octos);

                // Only count flashes up to step 100 for part 1.
                if step <= 100 { flashes += new_flashes; }
            }
        }

        // Go back through and clamp to 0.
        for y in 0..10 {
            for x in 0..10 {
                if octos[x][y] > 9 {
                    octos[x][y] = 0;
                }
            }
        }

        println!("\nAfter step {}:", step);
        print_octos(&octos);

        let all_zero: bool = octos.iter().fold(true, |zero_before, next_row| zero_before &&
            next_row.iter().all(|&octo| octo == 0)
        );
        if all_zero {
            // For part 2, stop when we find a full flash.
            break;
        }
    }

    print!("\nTotal flahes at step 100: {}\n", flashes);
}

fn load_octos(octos: &mut Octogrid) {
    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, num_char) in line.unwrap().chars().into_iter().enumerate() {
            octos[x][y] = num_char.to_digit(10).unwrap() as i8;
        }
    }

    println!("Before any steps:");
    print_octos(octos);
}

fn print_octos(octos: &Octogrid) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", octos[x][y]);
        }
        print!("\n");
    }
}

fn propagate_flashes((x, y): (usize, usize), mut octos: &mut Octogrid) -> u64 {
    let mut flash_count = 0;

    octos[x][y] += 1;
    if octos[x][y] == 10 {
        flash_count += 1;

        let min_x = if x > 0 { x - 1 } else { 0 };
        let min_y = if y > 0 { y - 1 } else { 0 };
        let max_x = cmp::min(9, x + 1);
        let max_y = cmp::min(9, y + 1);

        for y_next in min_y..=max_y {
            for x_next in min_x..=max_x {
                if x != x_next || y != y_next {
                    flash_count += propagate_flashes((x_next, y_next), &mut octos);
                }
            }
        }
    }

    return flash_count;
}
