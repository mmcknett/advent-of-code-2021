use std::io::{self, BufRead};

fn main() {
  let mut transients = vec![];
  let mut readout = vec![];

  load_stdin(&mut transients, &mut readout);

  let mut digit_counts: [u32; 10] = [0; 10];

  for r in readout {
    match r.len() {
      2 /*segments*/ => { digit_counts[1] += 1; print!("{} ", r) },
      4 /*segments*/ => { digit_counts[4] += 1; print!("{} ", r) },
      3 /*segments*/ => { digit_counts[7] += 1; print!("{} ", r) },
      7 /*segments*/ => { digit_counts[8] += 1; print!("{} ", r) },
      _ => ()
    }
  }

  let sum = digit_counts[1] + digit_counts[4] + digit_counts[7] + digit_counts[8];
  println!("\n1, 4, 7, or 8 appear {} times.", sum);
}

fn load_stdin(transients: &mut Vec<String>, readout: &mut Vec<String>)
{
  for line in io::stdin().lock().lines() {
    let l = line.unwrap();
    let halves: Vec<&str> = l.split('|').into_iter().collect();

    for value in halves[0].trim().split(" ") {
      transients.push(String::from(value));
    }

    for value in halves[1].trim().split(" ") {
      readout.push(String::from(value));
    }
  }
}