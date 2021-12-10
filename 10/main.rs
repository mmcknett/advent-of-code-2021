use std::io::{self, BufRead};

fn main() {
    let mut syntax_score: u64 = 0;
    for line in io::stdin().lock().lines() {
        let mut stack = vec![];
        for c in line.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let open = stack.pop().unwrap();
                    if !is_pair(open, c) {
                        println!("{} doesn't pair with {}", open, c);
                        syntax_score += score(c);
                    }
                },
                _ => panic!("Invalid character")
            }
        }
    }

    println!("Syntax score is: {}", syntax_score);
}

fn is_pair(open: char, close: char) -> bool {
    match open {
        '(' => return close == ')',
        '{' => return close == '}',
        '<' => return close == '>',
        '[' => return close == ']',
        _ => panic!("Invalid open character")
    }
}

fn score(close: char) -> u64 {
    match close {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => panic!("Invalid character")
    }
}
