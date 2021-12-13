use std::io::{self, BufRead};
use std::cmp;
use simple_grid::Grid;

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

type Pointgrid = DynGrid<bool>;

#[derive(Debug)]
struct Fold {
  vertical: bool,
  value: usize
}

fn main() {
  let mut pointgrid: Pointgrid = DynGrid::new(1500, 1500);
  let mut folds = vec![];

  for line in io::stdin().lock().lines() {
    let linestr = line.unwrap();

    if linestr == "" {
      continue;
    }

    if linestr.starts_with("fold along ") {

      let commandstr = linestr.strip_prefix("fold along ").unwrap();
      let mut split = commandstr.split("=");
      let vertical = match split.next().unwrap().chars().next().unwrap() {
        'y' => false,
        'x' => true,
        _ => panic!("Not a direciton!")
      };
      let value = split.next().unwrap().parse::<usize>().unwrap();
      folds.push(Fold { vertical, value });

    } else {

      let mut xy = linestr.split(",");
      let x = xy.next().unwrap().parse::<usize>().unwrap();
      let y = xy.next().unwrap().parse::<usize>().unwrap();
      pointgrid.set((x, y), true);
    }
  }

  // print_points(&points, &width, &height);
  println!("Folds: {:?}", folds);
  println!("Total dots visible: {}", count(&pointgrid));

  println!("Folding...");

  // Uncomment for part 1
  // fold(&mut pointgrid, &folds[0]);

  // Comment for loop & print for part 1.
  for a_fold in &folds {
    fold(&mut pointgrid, a_fold);
  }
  print_points(&pointgrid);
  
  println!("Total dots visible: {}", count(&pointgrid));
}

fn print_points(pointgrid: &Pointgrid) {
  pointgrid.print(|val: bool| if val { '#' } else { '.' });
}

fn count(grid: &Pointgrid) -> u32 {
  return grid.points.clone().subgrid(0, 0, grid.width, grid.height).cell_iter().fold(0, 
    |count, val: &bool| count + if *val { 1 } else { 0 }
  );
}

fn fold(grid: &mut Pointgrid, fold: &Fold) {
  if fold.vertical {
    for x in (fold.value + 1)..grid.width {
      for y in 0..grid.height {
        let x_refl = 2 * fold.value - x; // Reflect x across fold
        *grid.mut_u((x_refl, y)) = grid.get_u((x, y)) || grid.get_u((x_refl, y));
      }
    }

    grid.width = fold.value;
  } else {
    for y in (fold.value + 1)..grid.height {
      let y_refl = 2 * fold.value - y; // Reflect y across fold
      for x in 0..grid.width {
        *grid.mut_u((x, y_refl)) = grid.get_u((x, y)) || grid.get_u((x, y_refl));
      }
    }

    grid.height = fold.value;
  }
}
