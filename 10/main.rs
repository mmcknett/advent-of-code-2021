use std::io::{self, BufRead};

fn main() {
    let mut syntax_score: u64 = 0;
    let mut autocomplete_scores = vec![];

    for line in io::stdin().lock().lines() {
        let mut stack = vec![];
        let mut illegal = false;
        for c in line.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let open = stack.pop().unwrap();
                    if !is_pair(open, c) {
                        println!("{} doesn't pair with {}", open, c);
                        syntax_score += error_score(c);
                        illegal = true;
                        break;
                    }
                },
                _ => panic!("Invalid character")
            }
        }
        
        if !illegal {
            // It's a valid string, and we'll autocomplete (part 2)
            let mut autocomplete = 0;
            let mut result = String::new();
            while let Some(open_char) = stack.pop() {
                let close = get_match(open_char);
                result.push(close);
                autocomplete = 5 * autocomplete + autocomplete_score(close);
            }
            println!("{} - {} total points.", result, autocomplete);
            autocomplete_scores.push(autocomplete);
        }
    }

    autocomplete_scores.sort();
    println!("Syntax score is: {}", syntax_score);
    println!("The middle autocomplete score is: {}", autocomplete_scores[autocomplete_scores.len() / 2]);
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

fn error_score(close: char) -> u64 {
    match close {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid character")
    }
}

fn get_match(open: char) -> char {
    match open {
        '(' => ')',
        '{' => '}',
        '<' => '>',
        '[' => ']',
        _ => panic!("Invalid open character")
    }
}

fn autocomplete_score(close: char) -> u64 {
    match close {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid character")
    }
}
