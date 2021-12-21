use std::io::{self, BufRead};

// Use for sample.txt
const BOARD_SIZE: usize = 100;
const MAX_ENHANCES: usize = 50;
const PAD: usize = MAX_ENHANCES * 2 + BOARD_SIZE;
const SIZE: usize = BOARD_SIZE + 2 * PAD;

// Use for input.txt
// const SIZE: usize = 300;
// const PAD: usize = 102;

type Image = [[char; SIZE]; SIZE];

fn main() {
    let mut image = [['.'; SIZE]; SIZE];

    let mut algorithm = String::new();
    io::stdin().read_line(&mut algorithm).unwrap();

    for (y, line) in io::stdin().lock().lines().enumerate() {
        let line_str = line.unwrap();
        if line_str == "" { continue; }

        for (x, c) in line_str.chars().enumerate() {
            image[x + PAD][y + PAD] = c
        }
    }

    println!("Starting with...");
    paint(&image);
    println!("{} are lit.", num_lit(&image, 0));

    let mut pad = PAD; // Every time we enhance, we nee to expand outward 1 square.

    println!("Enhancing twice...");
    let enhanced = enhance(&image, &algorithm, &mut pad);
    // paint(&enhanced);

    let mut enhanced_twice = enhance(&enhanced, &algorithm, &mut pad);

    // paint(&enhanced_twice);
    // println!("{} are lit.", num_lit(&enhanced_twice, pad));



    // Removing the border doesn't solve the "infinite grid" problem.
    // ...
    // println!("Removing border artifact...");
    // remove_border_artifact(&mut enhanced_twice);
    // paint(&enhanced_twice);
    // println!("{} are lit.", num_lit(&enhanced_twice));



    // Part 2
    println!("Enhancing 48 more times...");
    let mut res: Image = enhanced_twice;
    for _ in 2..MAX_ENHANCES {
        res = enhance(&res, &algorithm, &mut pad);
        // remove_border_artifact(&mut res);
        // paint(&res);
        // println!("...");
    }

    paint(&res);
    
    // 20262 is too high.
    // 19524 is too low.
    println!("{} are lit. Pad is {}", num_lit(&res, pad), pad);
}

fn paint(image: &Image) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!("{}", image[x][y]);
        }
        println!("");
    }
}

fn num_lit(image: &Image, pad: usize) -> u32 {
    let mut sum = 0;
    for y in pad..(SIZE-pad) {
        for x in pad..(SIZE-pad) {
            sum += magnitude(x, y, image) as u32;
        }
    }
    return sum;
}

fn enhance(image: &Image, alg: &String, pad: &mut usize) -> Image {
    // Place a kernel on each pixel of the image
    // Use the kernel to calculate the index into alg
    // With the index, find the new pixel value by indexing into alg.
    let mut result: Image = [['.'; SIZE]; SIZE];

    *pad -= 2;
    // let p = *pad;
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let index = kernel(x, y, image);
            result[x][y] = alg.chars().nth(index).unwrap();
        }
    }

    return result;
}

fn kernel(x: usize, y: usize, image: &Image) -> usize {
    let x: i32 = x as i32;
    let y: i32 = y as i32; // The kernel applies to an "infinite" image

    let mut accum = 0;

    for k_y in (y-1)..=(y+1) {
        for k_x in (x-1)..=(x+1) {
            let bit = 
                if k_x < 0 || k_x >= SIZE as i32 || k_y < 0 || k_y >= SIZE as i32 {
                    0
                } else {
                    magnitude(k_x as usize, k_y as usize, image)
                };
            accum = (accum << 1) + bit;
        }
    }

    return accum;
}

fn magnitude(x: usize, y: usize, image: &Image) -> usize {
    if image[x][y] == '#' { 1 } else { 0 }
}

fn remove_border_artifact(image: &mut Image) {
    // Remove the border that got added
    for y in 0..SIZE {
        image[0][y] = '.';
        image[SIZE-1][y] = '.';
    }

    for x in 0..SIZE {
        image[x][0] = '.';
        image[x][SIZE-1] = '.';
    }
}
