use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
  let instructions = load();
  let (x, depth) = instructions.iter().fold(
    (0, 0),
    |(x, depth), next_inst| {
      match next_inst.dir {
        Dirs::Forward => (x + next_inst.mag, depth),
        Dirs::Up => (x, depth - next_inst.mag),
        Dirs::Down => (x, depth + next_inst.mag)
      }
    }
  );

  println!("x times depth is {}", x * depth);

  let (x, depth, _) = instructions.iter().fold(
    (0, 0, 0),
    |(x, depth, aim), next_inst| {
      match next_inst.dir {
        Dirs::Down => (x, depth, aim + next_inst.mag),
        Dirs::Up => (x, depth, aim - next_inst.mag),
        Dirs::Forward => (x + next_inst.mag, depth + aim * next_inst.mag, aim)
      }
    }
  );

  println!("Corrected x times depth is {}", x * depth);
}

fn load() -> Vec<Inst> {
  io::stdin().lock().lines().map(
    |s| s.unwrap().parse::<Inst>().unwrap()
  ).collect()
}

#[derive(Debug, Clone)]
struct Inst { 
  dir: Dirs,
  mag: u32
}

impl FromStr for Inst {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let v: Vec<&str> = s.split(" ").collect();
    Ok(Inst {
      dir: v[0].parse::<Dirs>().unwrap(),
      mag: v[1].parse::<u32>().unwrap()
    })
  }
}

#[derive(Debug, Clone)]
enum Dirs {
  Forward,
  Down,
  Up
}

impl FromStr for Dirs  {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "forward" => Ok(Dirs::Forward),
      "down" => Ok(Dirs::Down),
      "up" => Ok(Dirs::Up),
      _ => Err("Unknown direction".to_string())
    }
  }
}