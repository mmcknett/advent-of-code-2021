use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
  let mut caves = [[0; 1000]; 1000];
  let mut width = 0;
  let mut height = 0;

  load_caves(&mut caves, &mut width, &mut height);
  print_caves(&caves, &width, &height);

  // Part 1

  let mut sum_risk_level: u64 = 0;
  let mut low_point_list: Vec<(usize, usize)> = vec![];

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
        low_point_list.push((x, y));
        let risk_level = caves[x][y] as u64 + 1;
        sum_risk_level += risk_level;
      }
    }
  }
  println!("Risk level sum is: {}", sum_risk_level);

  // Part 2

  let mut visited = [[false; 1000]; 1000];
  let mut maxheap = BinaryHeap::new();

  for l in low_point_list {
    maxheap.push(dfs_basin_size(&caves, &mut visited, &width, &height, &l))
  }

  println!("Heap is {:?}", maxheap);
  let basin_product = maxheap.iter().take(3).fold(1, |product, val| product * val);
  println!("The product of the largest three basin sizes is: {}", basin_product);
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

fn dfs_basin_size(
  caves: &[[u8; 1000]; 1000],
  visited: &mut [[bool; 1000]; 1000],
  width: &usize,
  height: &usize,
  (x, y): &(usize, usize)) -> u32
{
  if visited[*x][*y] || caves[*x][*y] == 9 {
    return 0;
  }

  visited[*x][*y] = true;
  let mut size = 1;
  if *y > 0 { // up
    size += dfs_basin_size(caves, visited, width, height, &(*x, y-1));
  } 
  if y + 1 < *height { // down
    size += dfs_basin_size(caves, visited, width, height, &(*x, y+1));
  }
  if *x > 0 { // left
    size += dfs_basin_size(caves, visited, width, height, &(x-1, *y));
  }
  if x + 1 < *width { // right
    size += dfs_basin_size(caves, visited, width, height, &(x+1, *y));
  }

  return size;
}
