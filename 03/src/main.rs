use std::io;

// Stole this binary vector formatter from stack overflow
// https://stackoverflow.com/questions/54042984/can-i-format-debug-output-as-binary-when-the-values-are-in-a-vector
use std::fmt;

struct V(Vec<u16>);

// custom output
impl fmt::Binary for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() { 
            if count != 0 { write!(f, " ")?; }

            write!(f, "{:b}", n)?;
        }

        Ok(())
    }
}


// My code...

fn main() -> io::Result<()> {
    let (bitcount, bitmask, start_bit, input_numbers) = read_numbers();
    let (gamma, epsilon) = gamma_epsilon(&input_numbers, bitcount, bitmask, start_bit);

    println!("Gamma is {}", gamma);
    println!("Epsilon is {}", epsilon);

    println!("Power consumption is {}", gamma as u64 * epsilon as u64);

    // Part 2
    let mut ox_candidates = input_numbers.clone();

    let mut current_bit: u16 = start_bit;
    while current_bit > 0 {
        // Use gamma for Oxygen, because gamma's bit at current_bit is
        // the bit that has the most common bit of all the numbers
        let (gamma, _) = gamma_epsilon(&ox_candidates, bitcount, bitmask, start_bit);

        ox_candidates.retain(|x| x & current_bit == gamma & current_bit);
        if ox_candidates.len() <= 1 {
            break;
        }
        current_bit >>= 1;
        // println!("{:b}", V(ox_candidates.clone()));
    }

    let mut co2_candidates = input_numbers.clone();

    let mut current_bit: u16 = start_bit;
    while current_bit > 0 {
        // Use epsilon for CO2, because epsilon's bit at current_bit is
        // the bit that has the least common bit of all the numbers
        let (_, epsilon) = gamma_epsilon(&co2_candidates, bitcount, bitmask, start_bit);

        co2_candidates.retain(|x| x & current_bit == epsilon & current_bit);
        if co2_candidates.len() <= 1 {
            break;
        }
        current_bit >>= 1;
        // println!("{:b}", V(co2_candidates.clone()));
    }

    println!("Oxygen rating: {:?}", ox_candidates);
    println!("CO2 rating: {:?}", co2_candidates);
    println!("Life support rating: {}", ox_candidates[0] as u64 * co2_candidates[0] as u64);

    Ok(())
}

fn read_numbers() -> (usize, u16, u16, Vec<u16>) {
    let mut input_numbers = vec![];
    let mut bitmask = None;
    let mut start_bit = None;
    let mut input_size = 0;

    // Read in numbers
    loop {
        let mut input = String::new();
        let bytesread = io::stdin().read_line(&mut input).unwrap();
        if bytesread == 0 { break; }

        let trimmed_input = input.trim();
        input_size = trimmed_input.len();

        if bitmask.is_none() || start_bit.is_none() {
            let mut bitmask_inner = 0;
            for _ in 0..input_size {
                bitmask_inner = (bitmask_inner << 1) + 1;
            }
            bitmask = Some(bitmask_inner);

            start_bit = Some(1 << (input_size - 1));
        }

        input_numbers.push(u16::from_str_radix(trimmed_input, 2).unwrap());
    }

    return (input_size, bitmask.unwrap(), start_bit.unwrap(), input_numbers);
}

fn gamma_epsilon(number_list: &Vec<u16>, bitcount: usize, bitmask: u16, start_bit: u16) -> (u16, u16) {
    let mut bit_counts: Vec<u64> = Vec::with_capacity(bitcount);
    for _ in 0..bitcount { bit_counts.push(0); }

    // Sum up all of the entries in each part of gamma
    for &input in number_list {
        let mut current_bit: u16 = start_bit;
        for i in 0..bit_counts.len() {
            if current_bit & input > 0 {
                bit_counts[i] += 1
            };
            current_bit >>= 1;
        }
    }

    // println!("bit counts is {:?}", bit_counts);

    let run_len = number_list.len() as u64;
    let normalized_bit_counts = bit_counts.iter().map(|val| ((val + run_len / 2) / run_len) as u16);

    // println!("bit counts normalized is {:?}", normalized_bit_counts);

    let gamma: u16 = normalized_bit_counts.into_iter().reduce(|accum, val| (accum << 1) + val).unwrap();
    let epsilon: u16 = !gamma & bitmask;
    return (gamma, epsilon);
}
