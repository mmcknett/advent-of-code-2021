use std::io::{self, BufRead};
use std::cmp;
use simple_grid::{Grid, GridIndex};

type Points = Grid<bool>;

type Points = [[bool; 1500]; 1500];

#[derive(Debug)]
struct Fold {
  vertical: bool,
  value: usize
}

fn main() {
  let mut width: usize = 0;
  let mut height: usize = 0;
  let mut points: Points = Grid::new_default(1500, 1500);
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
      height = cmp::max(y+1, height);
      width = cmp::max(x+1, width);
      
      if height > 1500 || width > 1500 {
        panic!("{} x {} is too big!", width, height);
      }

      *points.get_mut((x, y)).unwrap() = true;
    }
  }

  // print_points(&points, &width, &height);
  println!("Folds: {:?}", folds);
  println!("Total dots visible: {}", count(&points, &width, &height));

  println!("Folding...");
  for a_fold in &folds {
    fold(&mut points, &mut width, &mut height, a_fold);
  }
  
  print_points(&points, &width, &height);
  println!("Total dots visible: {}", count(&points, &width, &height));
}

fn print_points(points: &Points, width: &usize, height: &usize) {
  for y in 0..*height {
    for x in 0..*width {
      print!("{}", if *points.get((x, y)).unwrap() { '#' } else { '.' });
    }
    print!("\n");
  }
  println!("({} x {})", width, height);
}

fn count(points: &Points, width: &usize, height: &usize) -> u32 {
  return points.clone().subgrid(0, 0, *width, *height).cell_iter().fold(0, |count, val: &bool| count + if *val { 1 } else { 0 });
}

fn fold(points: &mut Points, width: &mut usize, height: &mut usize, fold: &Fold) {
  if fold.vertical {
    for x in (fold.value + 1)..*width {
      for y in 0..*height {
        let reflected_x = 2 * fold.value - x;
        *points.get_mut((reflected_x, y)).unwrap() =
          *points.get((x, y)).unwrap() || 
          *points.get((reflected_x, y)).unwrap();
      }
    }

    *width = fold.value;
  } else {
    for y in (fold.value + 1)..*height {
      let reflected_y = 2 * fold.value - y;
      for x in 0..*width {
        *points.get_mut((x, reflected_y)).unwrap() =
          *points.get((x, y)).unwrap() || 
          *points.get((x, reflected_y)).unwrap();
      }
    }

    *height = fold.value;
  }
}
