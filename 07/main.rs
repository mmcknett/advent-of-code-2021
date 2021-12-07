use std::path::Path;
use std::fs::File;
use std::str::FromStr;
use std::fmt;
use std::cmp;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
  let mut crabs = vec![];

  // Load from the file listed here...
  // let reader = read_file("sample.txt");
  // load_vec(&mut crabs, reader);

  // Load from stdin...
  load_vec_stdin::<i64>(&mut crabs);

  println!("{:?}", crabs);

  let min_pos = *crabs.iter().min().unwrap();
  let max_pos = *crabs.iter().max().unwrap();

  let mut min_sum = i64::MAX;
  let mut min_sum_corrected = i64::MAX;
  for pos in min_pos..=max_pos {
    let sum = sum_differences(pos, &crabs);
    min_sum = cmp::min(sum, min_sum);

    let sum_corrected = sum_diffs_corrected(pos, &crabs);
    min_sum_corrected = cmp::min(sum_corrected, min_sum_corrected);
  }

  println!("Minimum energy is {}", min_sum);
  println!("Minimum energy corrected is {}", min_sum_corrected);

  return Ok(());
}

fn read_file(pathname: &str) -> BufReader<File> {
  let path = Path::new(pathname);

  let file = match File::open(path) {
    Ok(file) => file,
    _ => panic!("Couldn't open the file.")
  };

  let reader = BufReader::new(file);
  return reader;
}

fn load_vec<T>(vector: &mut Vec<T>, reader: BufReader<File>)
where
  T: FromStr,
  <T as FromStr>::Err: fmt::Debug
{
  for line in reader.lines() {
    for value in line.unwrap().split(',') {
      vector.push(value.parse::<T>().unwrap());
    }
  }
}

fn load_vec_stdin<T>(vector: &mut Vec<T>)
where
  T: FromStr,
  <T as FromStr>::Err: fmt::Debug
{
  for line in io::stdin().lock().lines() {
    for value in line.unwrap().split(',') {
      vector.push(value.parse::<T>().unwrap());
    }
  }
}

fn sum_differences(pos: i64, vector: &Vec<i64>) -> i64 {
  return vector.iter().fold(0i64, |sum, val| sum + (pos - val).abs());
}

fn sum_diffs_corrected(pos: i64, vector: &Vec<i64>) -> i64 {
  return vector.iter().fold(0i64, |sum, val| {
    let diff = (pos - val).abs();
    sum + diff * (diff + 1) / 2
  });
}
