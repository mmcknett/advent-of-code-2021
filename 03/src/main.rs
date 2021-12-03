use std::io;

fn main() -> io::Result<()> {
    let mut gammavec = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut run_len = 0;

    // Sum up all of the entries in each part of gamma
    loop {
        let mut input = String::new();
        let bytesread = io::stdin().read_line(&mut input)?;
        if bytesread == 0 { break; }

        run_len += 1;
        for (i, c) in input.chars().enumerate() {
            match c.to_digit(10) {
                Some(1) => gammavec[i] += c.to_digit(10).unwrap(),
                _ => continue
            }
        }
    }

    for i in 0..gammavec.len() {
        println!("{}, {}, {}", gammavec[i], run_len/2, run_len);
        gammavec[i] = (gammavec[i] + (run_len/2)) / run_len;
    }

    let gamma: String = gammavec.iter().map(|c| c.to_string()).collect();
    println!("You typed: {}", gamma);

    let gamma_int: u16 = u16::from_str_radix(&gamma, 2).unwrap();
    println!("Gamma is {}", gamma_int);

    // poor man's not operator
    let epsilonvec = gammavec.map(|digit| if digit == 0 { 1 } else { 0 });

    let epsilonstr: String = epsilonvec.iter().map(|c| c.to_string()).collect();
    let epsilon: u16 = u16::from_str_radix(&epsilonstr, 2).unwrap();
    // let epsilon: u8 = !gamma_int;
    println!("Epsilon is {}", epsilon);

    println!("Power consumption is {}", gamma_int as u64 * epsilon as u64);

    Ok(())
}
