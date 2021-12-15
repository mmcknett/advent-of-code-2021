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

    risk_grid.print(|val| (48u8 + (val as u8)) as char);

    let lowest_risk = find_min_risk_path(&risk_grid);
    println!("The lowest total risk path is: {}", lowest_risk);

    // Part 2
    
}

fn find_min_risk_path(risk_grid: &RiskGrid) -> u64{
  let mut visit_queue = VecDeque::new();
  visit_queue.push_back((0,0));

  // New Grid containing the minimum risks required to reach each point
  let mut min_grid = RiskGrid::new(risk_grid.width, risk_grid.height);

  while let Some(curr) = visit_queue.pop_front() {
    let (x, y) = curr;

    let mut push_candidate = |next: Coord| {
      // The "candidate" risk is the risk of entering the next square from the current square.
      let candidate_risk = min_grid.get_u(curr) + risk_grid.get_u(next);

      if min_grid.get_u(next) == 0 {
        // Not set yet. The minimum is the candidate risk. Also, we need to visit it.
        min_grid.set(next, candidate_risk);
        visit_queue.push_back(next);
      } else {
        // Already set. The minimum is the minimum of the current & candidate risks.
        min_grid.set(next, cmp::min(candidate_risk, min_grid.get_u(next)));
      }
    };

    // right
    if x + 1 < risk_grid.width {
      push_candidate((x+1, y));
    }

    // down
    if y + 1 < risk_grid.height {
      push_candidate((x, y+1));
    }
  }

  let lowest_risk = min_grid.get_u((min_grid.width - 1, min_grid.height - 1));
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
  
    fn mut_u(&mut self, (x, y): (usize, usize)) -> &mut T {
      self.points.get_mut((x, y)).unwrap()
    }
  
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
  
    fn print(&self, f: fn(T) -> char) {
      for y in 0..self.height {
        for x in 0..self.width {
          print!("{}", f(*self.points.get((x, y)).unwrap()));
        }
        print!("\n");
      }
      println!("({} x {})", self.width, self.height);
    }
  
    // TODO: Implement iterator over all cells, respecting width & height.
  
    // TODO: Implement iterator over rows
  }