use std::io::{self, BufRead};

fn main() {
  let depths = load_depths();
  let mut count_increases = 0;
  for i in 1..depths.len() {
    count_increases += if depths[i] > depths[i-1] { 1 } else { 0 };
  }
  println!("there are {} increases", count_increases);

  let mut sums = vec![];
  for i in 2..depths.len() {
    sums.push(depths[i] + depths[i-1] + depths[i-2]);
  }

  count_increases = 0;
  for i in 1..sums.len() {
    count_increases += if sums[i] > sums[i-1] { 1 } else { 0 };
  }
  println!("there are {} triple-sum increases", count_increases);
}

fn load_depths() -> Vec<i32> {
  io::stdin().lock().lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>()
}
