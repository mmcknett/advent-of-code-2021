use std::io::{self, BufRead};
use std::cmp;
use std::collections::VecDeque;
use simple_grid::Grid;

type RiskGrid = DynGrid<u64>;
type Coord = (usize, usize);

fn main() {
    let mut risk_grid = RiskGrid::new(100, 100);

    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, num_char) in line.unwrap().chars().into_iter().enumerate() {
            risk_grid.set((x, y), num_char.to_digit(10).unwrap() as u64);
        }
    }

    risk_grid.print(Box::new(|val, _| (48u8 + (val as u8)) as char));

    let mut lowest_path = vec![];
    let lowest_risk = find_min_risk_path(&risk_grid, &mut lowest_path);

    let print_cmd = Box::new(
      move |val, coord| if lowest_path.contains(&coord) { (48u8 + (val as u8)) as char } else { ' ' }
    );
    risk_grid.print(print_cmd);
    println!("The lowest total risk path is: {}", lowest_risk);

    // Part 2
    let mut risk_grid_expanded = RiskGrid::new(1000,1000);
    for y in 0..risk_grid.height {
      for x in 0..risk_grid.width {
        for y_off in 0..5usize {
          for x_off in 0..5usize {
            // Have to clamp risk values between 1 & 9. Let's use 0-8 for mod.
            let risk = risk_grid.get_u((x, y));
            let risk_in_offset = ((risk - 1) + y_off as u64 + x_off as u64) % 9 + 1;
            risk_grid_expanded.set(
              (
                risk_grid.width * x_off + x,
                risk_grid.height * y_off + y
              ),
              risk_in_offset
            );
          }
        }
      }
    }

    risk_grid_expanded.print(Box::new(|val, _| (48u8 + (val as u8)) as char));

    let mut short_path_expanded = vec![];
    let lowest_risk_expanded = find_min_risk_path(&risk_grid_expanded, &mut short_path_expanded);
    // Welp, 2908 is too high.

    // let print_cmd_again = Box::new(
    //   move |val, coord| if short_path_expanded.contains(&coord) { (48u8 + (val as u8)) as char } else { ' ' }
    // );
    // risk_grid_expanded.print(print_cmd_again);

    println!("The lowest total risk path is: {}", lowest_risk_expanded);
}

fn find_min_risk_path(risk_grid: &RiskGrid, short_path: &mut Vec<Coord>) -> u64{
  let mut visit_queue = VecDeque::new();
  visit_queue.push_back((0,0));

  // New Grid containing the minimum risks required to reach each point
  let mut min_grid = RiskGrid::new(risk_grid.width, risk_grid.height);
  let mut visited: DynGrid<bool> = DynGrid::new(risk_grid.width, risk_grid.height);

  for y in 0..risk_grid.height {
    for x in 0..risk_grid.width {
      min_grid.set((x, y), if x == 0 && y == 0 { 0 } else { u64::MAX })
    }
  }

  while let Some(curr) = visit_queue.pop_front() {
    if visited.get_u(curr) { continue };
    let (x, y) = curr;

    let mut push_candidate = |next: Coord| {
      // The "candidate" risk is the risk of entering the next square from the current square.
      let candidate_risk = min_grid.get_u(curr) + risk_grid.get_u(next);
      if candidate_risk < min_grid.get_u(next) {
        visited.set(next, false);
        min_grid.set(next, candidate_risk);
      }

      visit_queue.push_back(next);
    };

    // right
    if x + 1 < risk_grid.width {
      push_candidate((x+1, y));
    }

    // down
    if y + 1 < risk_grid.height {
      push_candidate((x, y+1));
    }

    // up
    if x > 0 {
      push_candidate((x-1, y));
    }

    // left
    if y > 0 {
      push_candidate((x, y-1));
    }

    visited.set((x,y), true);
  }

  // Return a backtrace, too.
  let mut backtrack = (min_grid.width - 1, min_grid.height - 1);
  println!("{}", min_grid.get_u(backtrack));
  short_path.push(backtrack);
  while backtrack != (0, 0) {
    let (x, y) = backtrack;
    if x > 0 && y > 0 {
      let up = (x, y - 1);
      let left = (x - 1, y);
      backtrack = if min_grid.get_u(up) < min_grid.get_u(left) { up } else { left };
    } else if x > 0 {
      backtrack = (x - 1, y);
    } else {
      backtrack = (x, y - 1);
    }
    println!("{}", min_grid.get_u(backtrack));
    short_path.push(backtrack);
  }

  let lowest_risk = min_grid.get_u((min_grid.width - 1, min_grid.height - 1));


  // println!("The lowest total risk path is: {}", short_path);

  return lowest_risk;
}


// Steal DynGrid from Day 13

struct DynGrid<T> {
    points: Grid<T>,
    width: usize,
    height: usize
  }
  
  impl<T> DynGrid<T> 
    where T : Default, T: Copy
  {
    fn new(width_cap: usize, height_cap: usize) -> DynGrid<T> {
      DynGrid {
        points: Grid::new_default(width_cap, height_cap),
        width: 0,
        height: 0
      }
    }
  
    fn get_u(&self, (x, y): (usize, usize)) -> T {
      *self.points.get((x, y)).unwrap()
    }
  
    // fn mut_u(&mut self, (x, y): (usize, usize)) -> &mut T {
    //   self.points.get_mut((x, y)).unwrap()
    // }
  
    fn set(&mut self, (x, y): (usize, usize), val: T) {
      self.height = cmp::max(y+1, self.height);
      self.width = cmp::max(x+1, self.width);
      
      if self.height > self.points.height() ||
         self.width > self.points.width()
      {
        panic!("{} x {} is too big!", self.width, self.height);
      }
  
      *self.points.get_mut((x, y)).unwrap() = val;
    }
  
    fn print(&self, f: Box<dyn Fn(T, (usize, usize)) -> char>) {
      for y in 0..self.height {
        for x in 0..self.width {
          print!("{}", f(*self.points.get((x, y)).unwrap(), (x, y)));
        }
        print!("\n");
      }
      println!("({} x {})", self.width, self.height);
    }
  
    // TODO: Implement iterator over all cells, respecting width & height.
  
    // TODO: Implement iterator over rows
  }