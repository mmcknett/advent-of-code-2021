use std::io::{self, BufRead};
use std::collections::HashMap;

type InsertionRules = HashMap<(char, char), char>;

fn main() {
  let mut prior_step = String::new();
  io::stdin().read_line(&mut prior_step).unwrap();
  prior_step = String::from(prior_step.trim_end());
  println!("Template:\t{}", prior_step);

  let mut rules: InsertionRules = HashMap::new();

  // let mut bigram_list = vec![];
  // let mut bigram_to_index: HashMap<&str, u8> = HashMap::new();
  // let mut bigram_rules: HashMap<&str, u8> = HashMap::new();

  for line in io::stdin().lock().lines() {
    let linestr = line.unwrap();

    if linestr == "" {
      continue;
    }

    let mut frto = linestr.split(" -> ");
    let fr = frto.next().unwrap();
    let to: char = frto.next().unwrap().chars().next().unwrap();

    let mut cs = fr.chars();
    let char_pair: (char, char) = (cs.next().unwrap(), cs.next().unwrap());

    if rules.contains_key(&char_pair) {
      panic!("Didn't expect duplicate keys");
    }

    rules.insert(char_pair, to);
  }

  // for step in 1..=10 {
  //   let mut step_result = String::from(&prior_step[0..1]);
  //   for i in 0..(prior_step.len() - 1) {
  //     let twochars = String::from(&prior_step[i..=i+1]);
  //     let insert_char = rules.get(&twochars).unwrap();
  //     step_result.push_str(insert_char);
  //     step_result.push_str(&twochars[1..=1]);
  //   }

  //   println!("{}\nAfter step {}: ({})", step_result, step, step_result.len());
  //   prior_step = step_result;
  // }

  // let char_counts = prior_step.chars().fold(HashMap::new(), |mut hash, c| {
  //   *hash.entry(c).or_insert(0u32) += 1; hash
  // });

  let start = &prior_step;

  counts_from_bigram_start(start, 1, &rules);
  counts_from_bigram_start(start, 2, &rules);
  let char_counts = counts_from_bigram_start(start, 3, &rules);

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

fn counts_from_bigram_start(
  start: &str,
  depth: u8,
  rules: &InsertionRules) -> HashMap<char, u32>
{
  let first_char = start.chars().next().unwrap();
  let mut char_counts = HashMap::from([
    (first_char, 1)
  ]);

  let mut left = start.chars();
  let mut right = start.chars();
  right.next();

  let mut strs_result = vec![];

  while let Some(c_r) = right.next() {
    *char_counts.entry(c_r).or_insert(0u32) += 1;
    let c_l = left.next().unwrap();
    let res = counts_from_bigram((c_l, c_r), depth, &rules, &mut char_counts);
    strs_result.push(res);
  }

  println!("{}{}", first_char, strs_result.join(""));

  return char_counts;
}

fn counts_from_bigram(
  bigram: (char, char),
  depth: u8,
  rules: &InsertionRules,
  char_counts: &mut HashMap<char, u32>) -> String
{
  if depth == 0 {
    return String::from(bigram.1);
  }

  let c = rules.get(&bigram).unwrap();
  *char_counts.entry(*c).or_insert(0u32) += 1;

  let left = counts_from_bigram((bigram.0, *c), depth - 1, &rules, char_counts);
  let right = counts_from_bigram((*c, bigram.1), depth - 1, &rules, char_counts);

  return left + &right;
}


