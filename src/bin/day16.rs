use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
  x: i32,
  y:i32
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

#[derive(Clone)]
enum Mirrors {
  VerticalSplit(bool),
  HorizontalSplit(bool),
  DiagDownRight,
  DiagDownLeft
}

fn main(){
  let vec = inp::parse_file("inputs/day16.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn do_lazer(start: Point, direction: Point, mirrors: & mut HashMap<Point, Mirrors>, size: (usize, usize)) -> HashSet<Point> {
  let mut curr_point = start;
  let mut curr_direction = direction;
  let mut energized = HashSet::new();
  while (0..size.0).contains(&(curr_point.x as usize)) && (0..size.1).contains(&(curr_point.y as usize)) {
    energized.insert(curr_point);
    match mirrors.get(&curr_point) {
      Some(mirror) => {
        match mirror {
          Mirrors::VerticalSplit(true) => {
            return energized;
          },
          Mirrors::HorizontalSplit(true) => {
            return energized;
          },
          Mirrors::VerticalSplit(false) => {
            if curr_direction.y == 0 {
              mirrors.insert(curr_point, Mirrors::VerticalSplit(true));
              energized = energized.union(&do_lazer(curr_point + Point{x:0,y:1}, Point{x:0,y:1}, mirrors, size)).map(|x|*x).collect();
              energized = energized.union(&do_lazer(curr_point + Point{x:0,y:-1}, Point{x:0,y:-1}, mirrors, size)).map(|x|*x).collect();
            } else {
              curr_point = curr_point + curr_direction;
            }
          }
          Mirrors::HorizontalSplit(false) => {
            if curr_direction.x == 0 {
              mirrors.insert(curr_point, Mirrors::HorizontalSplit(true));
              energized = energized.union(&do_lazer(curr_point + Point{x:1,y:0}, Point{x:1,y:0}, mirrors, size)).map(|x|*x).collect();
              energized = energized.union(&do_lazer(curr_point + Point{x:-1,y:0}, Point{x:-1,y:0}, mirrors, size)).map(|x|*x).collect();
            } else {
              curr_point = curr_point + curr_direction;
            }
          }
          Mirrors::DiagDownRight => {
            match curr_direction {
              Point{x:1,y:0} => {
                curr_direction = Point{x:0, y:1};
              },
              Point{x:-1,y:0} => {
                curr_direction = Point{x:0, y:-1};
              },
              Point{x:0,y:1} => {
                curr_direction = Point{x:1, y:0};
              },
              Point{x:0,y:-1} => {
                curr_direction = Point{x:-1, y:0};
              },
              _=>()
            }
            curr_point = curr_point + curr_direction;
          }
          Mirrors::DiagDownLeft => {
            match curr_direction {
              Point{x:1,y:0} => {
                curr_direction = Point{x:0, y:-1};
              },
              Point{x:-1,y:0} => {
                curr_direction = Point{x:0, y:1};
              },
              Point{x:0,y:1} => {
                curr_direction = Point{x:-1, y:0};
              },
              Point{x:0,y:-1} => {
                curr_direction = Point{x:1, y:0};
              },
              _=>()
            }
            curr_point = curr_point + curr_direction;
          }
        }
      },
      None => {
        curr_point = curr_point + curr_direction;
      }
    }

  }
  energized
}

fn parse_mirrors(input: &Vec<String>) -> HashMap<Point, Mirrors> {
  let mut mirrors = HashMap::new();
  for (y,line) in input.iter().enumerate() {
    for (x,character) in line.chars().enumerate() {
      let point = Point{x: x as i32, y: y as i32};
      match character {
        '|' => {mirrors.insert(point, Mirrors::VerticalSplit(false));},
        '-' => {mirrors.insert(point, Mirrors::HorizontalSplit(false));},
        '/' => {mirrors.insert(point, Mirrors::DiagDownLeft);},
        '\\' => {mirrors.insert(point, Mirrors::DiagDownRight);},
        _ => ()
      }
    }
  }
  mirrors
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> usize {
  let mut mirrors = parse_mirrors(input);
  let xsize = input[0].chars().collect::<Vec<_>>().len();
  let ysize = input.len();
  do_lazer(Point{x:0, y:0}, Point{x:1, y:0}, &mut mirrors, (xsize, ysize)).len()
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> usize {
  let mirrors = parse_mirrors(input);
  let xsize = input[0].chars().collect::<Vec<_>>().len();
  let ysize = input.len();
  let mut max_energized = 0;
  for x in 0..xsize {
    max_energized = std::cmp::max(max_energized, do_lazer(Point{x:x as i32, y:0}, Point{x:0,y:1}, &mut mirrors.clone(), (xsize, ysize)).len());
    max_energized = std::cmp::max(max_energized, do_lazer(Point{x:x as i32, y:ysize as i32-1}, Point{x:0,y:-1}, &mut mirrors.clone(), (xsize, ysize)).len());
  }
  for y in 0..ysize {
    max_energized = std::cmp::max(max_energized, do_lazer(Point{x:0, y:y as i32}, Point{x:1,y:0}, &mut mirrors.clone(), (xsize, ysize)).len());
    max_energized = std::cmp::max(max_energized, do_lazer(Point{x:xsize as i32 - 1, y:y as i32}, Point{x:-1,y:0}, &mut mirrors.clone(), (xsize, ysize)).len());
  }
  max_energized
}

#[cfg(test)]
mod day16_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(46, solve_part1(&inp::parse_file("test_inputs/day16_test.txt")));
    assert_eq!(51, solve_part2(&inp::parse_file("test_inputs/day16_test.txt")));
  }
}