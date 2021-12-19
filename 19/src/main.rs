use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let mut scanners = vec![];
    let mut scanner_num = 0;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                if let Some(_) = line_str.find("scanner") {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    scanner_num = usize::from_str(&cap[1]).unwrap();
                    scanners.push(vec![]);
                } else if line_str != "" {
                    let re = Regex::new(r"([-\d]+),([-\d]+),([-\d]+)").unwrap();
                    let cap = re.captures_iter(&line_str).next().unwrap();
                    let coord = (
                        i32::from_str(&cap[1]).unwrap(),
                        i32::from_str(&cap[2]).unwrap(),
                        i32::from_str(&cap[3]).unwrap()
                    );
                    scanners[scanner_num].push(coord);
                }
            },
            Err(_) => ()
        }
    }

    
}
