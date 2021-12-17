#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use std::io::{self, Read};
use num_traits::{FromPrimitive};

fn main() {
    // let buffer: Bitstream = vec![0xC, 0x2, 0x0, 0x0, 0xB, 0x4, 0x0, 0xA, 0x8, 0x2];
    let mut buffer: Bitstream = vec![];
    io::stdin().lock().read_to_end(&mut buffer);

    buffer = buffer.iter().filter(|&&c| c != 10 && c != 13).map(|c| match *c as char {
        '0'..='9' => (c - 48),
        'A'..='F' => (c - 55),
        _ => panic!("Not a hex digit")
    }).collect();

    let mut packet_parser = PacketParser {
        biter: Biter::new(&buffer),
        bits_consumed: 0,
        total_versions: 0
    };
    packet_parser.read_packet();

    println!("Version sum is: {}", packet_parser.total_versions);
}

struct PacketParser<'a> {
    biter: Biter<'a>,
    bits_consumed: u32,
    total_versions: u32
}

impl<'a> PacketParser<'a> {
    fn next(&mut self, bit_count: u8) -> u16 {
        self.bits_consumed += bit_count as u32;
        self.biter.next(bit_count)
    }

    fn read_packet(&mut self) {
        let header = self.read_header();
        println!("{:?}", header);

        
        if header.packet_type_id == Operator::Literal {
            self.read_literal_body();
        } else {
            self.read_operator_body();
        }
    }

    fn read_header(&mut self) -> Header {
        let version = self.next(3) as u8;
        let packet_type_id = Operator::from_u16(self.next(3)).unwrap();

        self.total_versions += version as u32;

        return Header {
            version,
            packet_type_id
        };
    }

    fn read_operator_body(&mut self) {
        let length_type = self.next(1);
        if length_type == 0 {
            let subpacket_size = self.next(15) as u32;

            let start = self.bits_consumed;
            while subpacket_size > (self.bits_consumed - start) {
                self.read_packet();
            }
        } else {
            let subpacket_count = self.next(11);

            for _ in 0..subpacket_count {
                self.read_packet();
            }
        }
    }

    fn read_literal_body(&mut self) {
        // Literals are 5-bit groups; a 1 indicates there
        // are more nibbles; a 0 indicates this is the last nibble.
        loop {
            let more = self.next(1) != 0;
            let _ = self.next(4); // Read a nibble
            if !more { break; }
        }
    }
}

#[derive(Debug)]
struct Header {
    version: u8,
    packet_type_id: Operator
}

#[repr(u8)]
#[derive(Primitive, Debug, Eq, PartialEq, Clone, Copy)]
enum Operator  {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    Equals = 7
}

type Bitstream = Vec<u8>;

trait Biterator {
    fn biter(&self) -> Biter;
}

struct Biter<'a> {
    nibble_vec: &'a Bitstream,
    vec_idx: usize,
    bit_idx: u8
}

impl Biterator for Bitstream {
    fn biter(&self) -> Biter {
        Biter::new(&self)
    }
}

impl<'a> Biter<'a> {
    fn new(nibble_vec: &Bitstream) -> Biter {
        Biter {
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

        if self.bit_idx != 0 {
            // Eat bits one at a time til the next nibble
            while self.bit_idx > 0 && bit_count > 0 {
                let offset = self.bit_idx - 1;
                let nextbit = ((0x1 << offset) & self.nibble_vec[self.vec_idx]) >> offset;
                result = (result << 1) + nextbit as u16;

                self.bit_idx -= 1;
                bit_count -= 1;
            }

            // We stopped because we consumed the last bit
            if self.bit_idx == 0 {
                self.vec_idx += 1;
            }
        }

        if bit_count > 0 {
            // Consume blocks of 4 bits, aligned on nibble boundaries.
            while bit_count >= 4 {
                let nibble = self.nibble_vec[self.vec_idx];
                result = (result << 4) + nibble as u16;

                self.vec_idx += 1;
                bit_count -= 4;
            }

            // Consume the remaining bits
            if self.vec_idx < self.nibble_vec.len() {
                let nibble = self.nibble_vec[self.vec_idx];
                result = (result << bit_count) +  match bit_count {
                    0 => { self.bit_idx = 0; 0 },
                    1 => { self.bit_idx = 3; (nibble as u16 & 0x8) >> 3 },
                    2 => { self.bit_idx = 2; (nibble as u16 & 0xC) >> 2 },
                    3 => { self.bit_idx = 1; (nibble as u16 & 0xE) >> 1 },
                    _ => panic!("This bitcount shouldn't be possible!")
                };
            } else if bit_count != 0 {
                panic!("We ran out of vector, but there were still requested bits!");
            }
        }

        return result;
    }
}

#[cfg(test)] 
mod tests {
    use super::{Bitstream, Biterator};

    const LITERAL_PACKET: [u8; 6] = [0b1101, 0b0010, 0b1111, 0b1110, 0b0010, 0b1000];
    const OPERATOR_PACKET: [u8; 14] = [0x3, 0x8, 0x0, 0x0, 0x6, 0xF, 0x4, 0x5, 0x2, 0x9, 0x1, 0x2, 0x0, 0x0];
    const OPERATOR_PACKET_V4: [u8; 18] = [0x8, 0xA, 0x0, 0x0, 0x4, 0xA, 0x8, 0x0, 0x1, 0xA, 0x8, 0x0, 0x0, 0x2, 0xF, 0x4, 0x7, 0x8];

    #[test]
    fn bitstream_gets_correct_bits() {
        let bitstream: Bitstream = Bitstream::from(LITERAL_PACKET);
        let mut biter = bitstream.biter();
        assert_eq!(6, biter.next(3));
        assert_eq!(4, biter.next(3));
        assert_eq!(0b10111, biter.next(5));
        assert_eq!(0b11110, biter.next(5));
        assert_eq!(0b00101, biter.next(5));
    }

    #[test]
    fn bitstream_gets_correct_bits_operator() {
        let bitstream: Bitstream = Bitstream::from(OPERATOR_PACKET);
        let mut biter = bitstream.biter();
        assert_eq!(1, biter.next(3));
        assert_eq!(6, biter.next(3));
        assert_eq!(0, biter.next(1));
        assert_eq!(27, biter.next(15));
        assert_eq!(0x68a, biter.next(11));
        assert_eq!(0x5224, biter.next(16));
    }
}
