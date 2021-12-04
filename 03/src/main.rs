use std::io;

fn main() -> io::Result<()> {
    let mut bit_counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let bitmask: u16 = 0b0000111111111111; // 12 bits
    let mut run_len = 0;

    // Sum up all of the entries in each part of gamma
    loop {
        let mut input = String::new();
        let bytesread = io::stdin().read_line(&mut input)?;
        if bytesread == 0 { break; }

        run_len += 1;
        for (i, c) in input.chars().enumerate() {
            match c.to_digit(10) {
                Some(1) => bit_counts[i] += 1,
                _ => continue
            }
        }
    }

    let normalized_bit_counts = bit_counts.map(|val| (val + run_len / 2) / run_len);
    let gamma: u16 = normalized_bit_counts.into_iter().reduce(|accum, val| (accum << 1) + val).unwrap();

    println!("Gamma is {}", gamma);

    let epsilon: u16 = !gamma & bitmask;
    println!("Epsilon is {}", epsilon);

    println!("Power consumption is {}", gamma as u64 * epsilon as u64);

    Ok(())
}
