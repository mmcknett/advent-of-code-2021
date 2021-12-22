use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;


fn main() {
    let (expected_on, instructions) = load_instructions();

    if expected_on.is_some() {
        println!("Expecting {} illuimnated", expected_on.unwrap());
    }

    for i in instructions {
        println!("{:?}", i);
    }
}

fn load_instructions() -> (Option<u32>, Vec<Instruction>) {
    let mut expected_on = None;
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                if let Some(_) = line_str.find("expect") {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    expected_on = Some(u32::from_str(&cap[1]).unwrap());
                }
                else if line_str != "" {
                    let re = Regex::new(r"(on|off) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    let inst = Instruction {
                        light_state: Lightstate::from_str(&cap[1]).unwrap(),
                        x_rg: normalize(i32::from_str(&cap[2]).unwrap(), i32::from_str(&cap[3]).unwrap()),
                        y_rg: normalize(i32::from_str(&cap[4]).unwrap(), i32::from_str(&cap[5]).unwrap()),
                        z_rg: normalize(i32::from_str(&cap[6]).unwrap(), i32::from_str(&cap[7]).unwrap())
                    };
                    instructions.push(inst);
                }
            },
            Err(_) => ()
        }
    }

    return (expected_on, instructions);
}

#[derive(Debug, Eq, PartialEq)]
enum Lightstate {
    On,
    Off
}

impl FromStr for Lightstate {
    type Err = ();

    fn from_str(input: &str) -> Result<Lightstate, Self::Err> {
        match input {
            "on"  => Ok(Lightstate::On),
            "off"  => Ok(Lightstate::Off),
            _      => Err(()),
        }
    }
}

fn normalize(a1: i32, a2: i32) -> (i32, i32) {
    if a1 < a2 { (a1, a2) } else { (a2, a1) }
}

#[derive(Debug)]
struct Instruction {
    light_state: Lightstate,
    x_rg: (i32, i32),
    y_rg: (i32, i32),
    z_rg: (i32, i32)
}
