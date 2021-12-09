use std::io::{self, BufRead};

fn main() {
  let mut caves = [[0; 1000]; 1000];
  let mut width = 0;
  let mut height = 0;

  load_caves(&mut caves, &mut width, &mut height);
  print_caves(&caves, &width, &height);

  let mut sum_risk_level: u64 = 0;

  for y in 0..height { 
    for x in 0..width {
      let mut low_point = true;
      if y > 0 { // up
        low_point = low_point && (caves[x][y-1] > caves[x][y]);
      } 
      if y + 1 < height { // down
        low_point = low_point && (caves[x][y+1] > caves[x][y]);
      }
      if x > 0 { // left
        low_point = low_point && (caves[x-1][y] > caves[x][y]);
      }
      if x + 1 < width { // right
        low_point = low_point && (caves[x+1][y] > caves[x][y]);
      }

      if low_point {
        println!("Low point: {} at {}, {}", caves[x][y], x, y);
        let risk_level = caves[x][y] as u64 + 1;
        sum_risk_level += risk_level;
      }
    }
  }
  println!("Risk level sum is: {}", sum_risk_level);
}

fn load_caves(caves: &mut [[u8; 1000]; 1000], width: &mut usize, height: &mut usize) {
  for (y, line) in io::stdin().lock().lines().enumerate() {
    *height += 1;
    for (x, num_char) in line.unwrap().chars().into_iter().enumerate() {
      caves[x][y] = num_char.to_digit(10).unwrap() as u8;
      if y == 0 { *width += 1 };
    }
  }

  println!("Loaded a {}x{} board", width, height);
}

fn print_caves(caves: &[[u8; 1000]; 1000], width: &usize, height: &usize) {
  println!("{}", caves.iter().take(*width).map(
    |s| s.iter().take(*height).map(
      |x| x.to_string()).collect::<String>()
    ).collect::<Vec<String>>().join("\n"
  ));
}
