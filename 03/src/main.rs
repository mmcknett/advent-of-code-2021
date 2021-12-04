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

// static BITMASK: u16 = 0x0FFF; // 12 bits
// static START_BIT: u16 = 0x0800;

static BITMASK: u16 = 0x001F; // 5 bits
static START_BIT: u16 = 0x0010;

fn main() -> io::Result<()> {
    let input_numbers = read_numbers();
    let (gamma, epsilon) = gamma_epsilon(&input_numbers);

    println!("Gamma is {}", gamma);
    println!("Epsilon is {}", epsilon);

    println!("Power consumption is {}", gamma as u64 * epsilon as u64);

    // Part 2
    let mut ox_candidates = input_numbers.clone();
    // for x in &mut input_numbers { let q = x; ox_candidates.push(q.clone()); } // Why is copying so freaking hard in rust...

    let mut current_bit: u16 = START_BIT;
    while current_bit > 0 {
        ox_candidates.retain(|x| x & current_bit > 0);
        if ox_candidates.len() <= 1 {
            break;
        }
        current_bit >>= 1;
        println!("{:b}", V(ox_candidates.clone()));
    }

    Ok(())
}

fn read_numbers() -> Vec<u16> {
    let mut input_numbers = vec![];

    // Read in numbers
    loop {
        let mut input = String::new();
        let bytesread = io::stdin().read_line(&mut input).unwrap();
        if bytesread == 0 { break; }

        input_numbers.push(u16::from_str_radix(&input.trim(), 2).unwrap());
    }

    return input_numbers;
}

fn gamma_epsilon(number_list: &Vec<u16>) -> (u16, u16) {
    // For use with input.txt
    // let mut bit_counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    // For use with sample.txt
    let mut bit_counts = [0, 0, 0, 0, 0];

    // Sum up all of the entries in each part of gamma
    for &input in number_list {
        let mut current_bit: u16 = START_BIT;
        for i in 0..bit_counts.len() {
            if current_bit & input > 0 {
                bit_counts[i] += 1
            };
            current_bit >>= 1;
        }
    }

    println!("bit counts is {:?}", bit_counts);

    let run_len = number_list.len() as u16;
    let normalized_bit_counts = bit_counts.map(|val| (val + run_len / 2) / run_len);

    println!("bit counts normalized is {:?}", normalized_bit_counts);


    let gamma: u16 = normalized_bit_counts.into_iter().reduce(|accum, val| (accum << 1) + val).unwrap();
    let epsilon: u16 = !gamma & BITMASK;
    return (gamma, epsilon);
}
