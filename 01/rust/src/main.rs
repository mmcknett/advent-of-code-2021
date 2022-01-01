use std::io::{self, BufRead};

fn main() {
  let depths = load_depths();
  let count = count_increases(&depths);
  println!("there are {} increases", count);

  let sums = triple_sums(&depths);
  let count = count_increases(&sums);
  println!("there are {} triple-sum increases", count);
}

fn load_depths() -> Vec<i32> {
  io::stdin()
    .lock()
    .lines()
    .map(
      |s| s.unwrap().parse::<i32>().unwrap()
    ).collect::<Vec<i32>>()
}

fn count_increases(list: &Vec<i32>) -> u32 {
  list
    .windows(2)
    .fold(
      0u32,
      |accum, slice| accum + if slice[1] > slice[0] { 1 } else { 0 }
    )
}

fn triple_sums(list: &Vec<i32>) -> Vec<i32> {
  list
    .windows(3)
    .map(|slice| slice.iter().sum())
    .collect()
}
