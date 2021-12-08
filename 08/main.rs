use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn main() {
  let puzzles = load_stdin();

  // --- Part 1 ---
  let mut digit_counts: [u32; 10] = [0; 10];

  for (_, readout) in &puzzles {
    for reading in readout {
      match reading.len() {
        2 /*segments*/ => { digit_counts[1] += 1 },
        4 /*segments*/ => { digit_counts[4] += 1 },
        3 /*segments*/ => { digit_counts[7] += 1 },
        7 /*segments*/ => { digit_counts[8] += 1 },
        _ => ()
      }
    }
  }

  let sum = digit_counts[1] + digit_counts[4] + digit_counts[7] + digit_counts[8];
  println!("\n1, 4, 7, or 8 appear {} times.", sum);

  // --- Part 2 ---

  // This checked that the puzzles are guaranteed to have 1s and 7s.
  // for (t, _) in &puzzles {
  //   if !t.iter().any(|s| s.len() == 2) {
  //     println!("len 2 transient missing", );
  //   }
  //   if !t.iter().any(|s| s.len() == 3) {
  //     println!("len 3 transient missing", );
  //   }
  // }

  // this checked that there are always ten values in the transients.
  // for (t, _) in &puzzles {
  //   if t.len() != 10 {
  //     println!("It's not 10");
  //   }
  // }

  // Segments
  //  000
  // 1   2
  //  333
  // 4   5
  //  666

  let mut final_sum: u64 = 0;

  for (transient, readout) in &puzzles {
    // println!("{:?}", transient);
    let mut segments: [char; 7] = [' '; 7];
    let one:   BTreeSet<char> = transient.iter().find(|&t| t.len() == 2).unwrap().chars().collect();
    let seven: BTreeSet<char> = transient.iter().find(|&t| t.len() == 3).unwrap().chars().collect();
    let four:  BTreeSet<char> = transient.iter().find(|&t| t.len() == 4).unwrap().chars().collect();
    let eight: BTreeSet<char> = transient.iter().find(|&t| t.len() == 7).unwrap().chars().collect();

    // println!("\nOne is {:?}, Seven is {:?}", one, seven);
    let seg_0 = seven.difference(&one).cloned().collect::<Vec<char>>()[0];
    // println!("Diff: {:?}", seg_0);

    segments[0] = seg_0;
    // println!("Segments: {:?}", segments);

    let five_seg_vecs = transient.iter().filter(|t| t.len() == 5).map(|t| t.clone()).collect::<Vec<String>>();
    let five_segs: Vec<BTreeSet<char>> = five_seg_vecs.into_iter().map(|s| s.chars().collect()).collect();

    // println!("Five segs is {:?}", five_segs);
    let three: &BTreeSet<char> = five_segs.iter().find(|&set| set.intersection(&one).collect::<Vec<&char>>().len() == 2).unwrap();
    let seg_1 = four.difference(three).cloned().collect::<Vec<char>>()[0];
    segments[1] = seg_1;
    // println!("Segments: {:?}", segments);

    let five: &BTreeSet<char> = five_segs.iter().find(|&set| set.contains(&segments[1])).unwrap();
    let two: &BTreeSet<char> = five_segs.iter().find(|&set| set != five && set != three).unwrap();
    segments[3] = *four.iter().find(|&&c| c != seg_1 && !one.contains(&c)).unwrap();
    // println!("Segments: {:?}", segments);

    let six_seg_vecs = transient.iter().filter(|t| t.len() == 6).map(|t| t.clone()).collect::<Vec<String>>();
    let six_segs: Vec<BTreeSet<char>> = six_seg_vecs.into_iter().map(|s| s.chars().collect()).collect();

    // println!("Six segs is {:?}", six_segs);
    let zero: &BTreeSet<char> = six_segs.iter().find(|&set| !set.contains(&segments[3])).unwrap();
    let nine: &BTreeSet<char> = six_segs.iter().find(|&set| set != zero && set.intersection(&one).collect::<Vec<&char>>().len() == 2).unwrap();
    let six: &BTreeSet<char> = six_segs.iter().find(|&set| set != zero && set != nine).unwrap();
    // println!("Zero: {:?}", zero);
    // println!("Nine: {:?}", nine);
    // println!("Six: {:?}", six);

    let mut result: u64 = 0;

    for read in readout {
      let read_set: BTreeSet<char> = read.chars().collect();
      // print!("{} ", read);
      let addend =
        if read_set == one { 1 }
        else if read_set == *two { 2 }
        else if read_set == *three { 3 }
        else if read_set == four { 4 }
        else if read_set == *five { 5 }
        else if read_set == *six { 6 }
        else if read_set == seven { 7 }
        else if read_set == eight { 8 }
        else if read_set == *nine { 9 }
        else if read_set == *zero { 0 }
        else { panic!("Didn't find a digit.") };

      result = result * 10 + addend;
    }

    println!("{:?} is {}", readout, result);
    final_sum += result;
  }

  println!("Sum of all output: {}", final_sum);
}

fn load_stdin() -> Vec<(Vec<String>, Vec<String>)>
{
  let mut puzzles = vec![];

  for line in io::stdin().lock().lines() {
    let l = line.unwrap();
    let halves: Vec<&str> = l.split('|').into_iter().collect();

    let mut transients: Vec<String> = vec![];
    let mut readout: Vec<String> = vec![];

    for value in halves[0].trim().split(" ") {
      transients.push(String::from(value));
    }

    for value in halves[1].trim().split(" ") {
      readout.push(String::from(value));
    }
    puzzles.push((transients, readout));
  }
  return puzzles;
}