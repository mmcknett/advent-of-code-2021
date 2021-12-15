use std::io::{self, BufRead};
use std::collections::HashMap;

type InsertionRules = HashMap<(char, char), char>;

const STEPS: u8 = 40;

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
    let to: char = frto.next().unwrap().chars().next().unwrap();

    let mut cs = fr.chars();
    let char_pair: (char, char) = (cs.next().unwrap(), cs.next().unwrap());

    if rules.contains_key(&char_pair) {
      panic!("Didn't expect duplicate keys");
    }

    rules.insert(char_pair, to);
  }

  let start = &prior_step;
  let char_counts = counts_from_bigram_start(start, STEPS, &rules);

  let max_char: (&char, &u64) = char_counts
    .iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, v)| (k, v))
    .unwrap();

  let min_char: (&char, &u64) = char_counts
    .iter()
    .min_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, v)| (k, v))
    .unwrap();

  println!("Max: ({}, {})", max_char.0, max_char.1);
  println!("Min: ({}, {})", min_char.0, min_char.1);
  let diff = max_char.1 - min_char.1;
  println!("Desired difference: {}", diff);
}

type Counts = HashMap<char, u64>;
type CountCache = HashMap<(char, char, u8), Counts>;

fn counts_from_bigram_start(
  start: &str,
  depth: u8,
  rules: &InsertionRules) -> Counts
{
  let first_char = start.chars().next().unwrap();
  let mut char_counts = HashMap::from([
    (first_char, 1)
  ]);

  let mut left = start.chars();
  let mut right = start.chars();
  right.next();

  let mut cache = CountCache::new();

  while let Some(c_r) = right.next() {
    *char_counts.entry(c_r).or_insert(0u64) += 1;
    let c_l = left.next().unwrap();
    if let Some(res) = counts_from_bigram((c_l, c_r), depth, &rules, &mut cache) {
      add_values(&mut char_counts, &res);
    }
  }

  return char_counts;
}

fn counts_from_bigram(
  bigram: (char, char),
  depth: u8,
  rules: &InsertionRules,
  cache: &mut CountCache) -> Option<Counts>
{
  if depth == 0 {
    return None;
  }

  let cache_key = (bigram.0, bigram.1, depth);
  if cache.contains_key(&cache_key) {
    return Some(cache.get(&cache_key).unwrap().clone());
  }

  let c = rules.get(&bigram).unwrap();
  let mut char_counts = HashMap::from([(*c, 1u64)]);

  if let Some(left) = counts_from_bigram((bigram.0, *c), depth - 1, &rules, cache) {
    add_values(&mut char_counts, &left);
  }

  if let Some(right) = counts_from_bigram((*c, bigram.1), depth - 1, &rules, cache) {
    add_values(&mut char_counts, &right);
  }

  cache.insert(cache_key, char_counts.clone());
  return Some(char_counts)
}

fn add_values(dest: &mut Counts, src: &Counts) {
  for (key, value) in src.iter() {
    *dest.entry(*key).or_insert(0u64) += value;
  }
}
