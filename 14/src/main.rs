use std::io::{self, BufRead};
use std::collections::HashMap;

type InsertionRules = HashMap<String, String>;

fn main() {
  let mut prior_step = String::new();
  io::stdin().read_line(&mut prior_step).unwrap();
  prior_step = String::from(prior_step.trim_end());
  println!("Template:\t{}", prior_step);

  let mut rules: InsertionRules = HashMap::new();

  for line in io::stdin().lock().lines() {
    let linestr = line.unwrap();

    if linestr == "" {
      continue;
    }

    let mut frto = linestr.split(" -> ");
    let fr = frto.next().unwrap();
    let to = frto.next().unwrap();

    if rules.contains_key(fr) {
      panic!("Didn't expect duplicate keys");
    }

    rules.insert(String::from(fr), String::from(to));
  }

  for step in 1..=10 {
    let mut step_result = String::from(&prior_step[0..1]);
    for i in 0..(prior_step.len() - 1) {
      let twochars = String::from(&prior_step[i..=i+1]);
      let insert_char = rules.get(&twochars).unwrap();
      step_result.push_str(insert_char);
      step_result.push_str(&twochars[1..=1]);
    }

    println!("{}\nAfter step {}: ({})", step_result, step, step_result.len());
    prior_step = step_result;
  }

  let char_counts = prior_step.chars().fold(HashMap::new(), |mut hash, c| {
    *hash.entry(c).or_insert(0u32) += 1; hash
  });

  let max_char: (&char, &u32) = char_counts
    .iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, v)| (k, v))
    .unwrap();

  let min_char: (&char, &u32) = char_counts
    .iter()
    .min_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, v)| (k, v))
    .unwrap();

  println!("Max: ({}, {})", max_char.0, max_char.1);
  println!("Min: ({}, {})", min_char.0, min_char.1);
  let diff = max_char.1 - min_char.1;
  println!("Desired difference: {}", diff);
}


