use std::io::{self, Read};

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    io::stdin().lock().read_to_end(&mut buffer);

    buffer = buffer.iter().filter(|&&c| c != 10 && c != 13).map(|c| match *c as char {
        '0'..='9' => (c - 48),
        'A'..='F' => (c - 55),
        _ => panic!("Not a hex digit")
    }).collect();

    println!("{:?}", buffer);
}

const VERSION_MASK: u64 = 0xE00000;

fn extract_version(packet: &Vec<u8>) -> u8 {
    let version = (packet[0] & 0xE) >> 1;
    return version;
}

struct Bitstream {
    nibble_vec: Vec<u8>,
    vec_idx: usize,
    bit_idx: u8
}

impl Bitstream {
    fn new(nibble_vec: Vec<u8>) -> Bitstream {
        Bitstream {
            nibble_vec: nibble_vec,
            vec_idx: 0,
            bit_idx: 0,
        }
    }

    fn next(&mut self, bit_count: u8) -> u16 {
        if bit_count == 0 || bit_count > 16 {
            panic!("Can't consume more than 16 bits at a time");
        }

        let mut bit_count = bit_count;
        let mut result = 0u16;

        // Cases to deal with...
        // 1 bit requested, bit_idx is 3 -> 2, 2 -> 1 or 1 -> 0
        // 2 bits requested, bit_idx is 3 -> 1 or 2 -> 0
        // 3 bits requested, bit_idx is 3 -> 0

        // Consume the bits remaining in the current nibble first.
        let bitmask = match bit_count {
            1 => 0x1,
            2 => 0x3,
            3 => 0x7,
            0 => 0xF,
            _ => panic!("WAT")
        };


        // Consume blocks of 4 bits, aligned on nibble boundaries.
        while bit_count >= 4 {
            let nibble = self.nibble_vec[self.vec_idx];
            result = (result << 4) + nibble as u16;

            self.vec_idx += 1;
            bit_count -= 4;
        }

        // Consume the remaining bits
        let nibble = self.nibble_vec[self.vec_idx];
        result += match bit_count {
            0 => { self.bit_idx = 0; 0 },
            1 => { self.bit_idx = 3; (nibble as u16 & 0x8) >> 3 },
            2 => { self.bit_idx = 2; (nibble as u16 & 0xC) >> 2 },
            3 => { self.bit_idx = 1; (nibble as u16 & 0xE) >> 1 },
            _ => panic!("This bitcount shouldn't be possible!")
        };

        return result;
    }
}

#[cfg(test)] 
mod tests {
    use super::{extract_version, Bitstream};

    const LITERAL_PACKET: [u8; 6] = [0b1101, 0b0010, 0b1111, 0b1110, 0b0010, 0b1000];
    const OPERATOR_PACKET: [u8; 14] = [0x3, 0x8, 0x0, 0x0, 0x6, 0xF, 0x4, 0x5, 0x2, 0x9, 0x1, 0x2, 0x0, 0x0];
    const OPERATOR_PACKET_V4: [u8; 18] = [0x8, 0xA, 0x0, 0x0, 0x4, 0xA, 0x8, 0x0, 0x1, 0xA, 0x8, 0x0, 0x0, 0x2, 0xF, 0x4, 0x7, 0x8];

    #[test]
    fn bitstream_gets_correct_bits() {
        let mut bitstream = Bitstream::new(Vec::from(LITERAL_PACKET));
        assert_eq!(6, bitstream.next(3));
        assert_eq!(4, bitstream.next(3));
        assert_eq!(0b10111, bitstream.next(5));
        assert_eq!(0b11110, bitstream.next(5));
        assert_eq!(0b00101, bitstream.next(5));
    }

    #[test]
    fn it_extracts_the_version() {
        let actual = extract_version(&Vec::from(LITERAL_PACKET));
        assert_eq!(6, actual);
    }

    #[test]
    fn it_extracts_the_version_for_operator_packet() {
        let actual = extract_version(&Vec::from(OPERATOR_PACKET));
        assert_eq!(1, actual);
    }

    #[test]
    fn it_extracts_the_version_for_operator_packet_v4() {
        let actual = extract_version(&Vec::from(OPERATOR_PACKET_V4));
        assert_eq!(4, actual);
    }
}
