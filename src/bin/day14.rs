use std::vec::Vec;
use std::collections::HashSet;
use std::ops::Add;

const CYCLES: i32 = 1000000000;

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

fn main(){
  let vec = inp::parse_file("inputs/day14.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_new_rock_position(start: Point, immovables: &HashSet<Point>, move_dir: Point, xsize: i32, ysize: i32) -> Point {
  let mut curr = start;
  let xrange = 0..xsize;
  let yrange = 0..ysize;
  while xrange.contains(&curr.x) && yrange.contains(&curr.y) {
    let new_pos = curr + move_dir;
    if immovables.contains(&new_pos) || !xrange.contains(&new_pos.x) || !yrange.contains(&new_pos.y) {
      return curr;
    }
    curr = new_pos
  }
  curr
}

fn move_all_rocks(movable_rocks: &HashSet<Point>, direction: Point, immovables: &HashSet<Point>, xsize: i32, ysize: i32) -> HashSet<Point> {
  let mut result = immovables.clone();
  match direction {
    Point{x:0, y:-1} => {
      for y in 0..ysize {
        for x in 0..xsize {
          match movable_rocks.get(&Point{x, y}) {
            Some(rock) => {
              result.insert(get_new_rock_position(*rock, &result,
                direction, xsize, ysize));
            },
            None => ()
          }
        }
      }

    },
    Point{x:0, y:1} => {
      for y in (0..ysize).rev() {
        for x in 0..xsize {
          match movable_rocks.get(&Point{x, y}) {
            Some(rock) => {
              result.insert(get_new_rock_position(*rock, &result,
                direction, xsize, ysize));
            },
            None => ()
          }
        }
      }

    },
    Point{x:-1, y:0} => {
      for x in 0..xsize {
        for y in 0..ysize {
          match movable_rocks.get(&Point{x, y}) {
            Some(rock) => {
              result.insert(get_new_rock_position(*rock, &result,
                direction, xsize, ysize));
            },
            None => ()
          }
        }
      }

    },
    Point{x:1, y:0} => {
      for x in (0..xsize).rev() {
        for y in 0..ysize {
          match movable_rocks.get(&Point{x, y}) {
            Some(rock) => {
              result.insert(get_new_rock_position(*rock, &result,
                direction, xsize, ysize));
            },
            None => ()
          }
        }
      }
    },
    _ => ()
  }
  result.difference(&immovables).map(|x| *x).collect()
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut movable_rocks = HashSet::new();
  let mut immovable_rocks = HashSet::new();
  for (y,line) in input.iter().enumerate() {
    for (x,entry) in line.chars().enumerate() {
      match entry {
        '#' => {immovable_rocks.insert(Point{x: x as i32,y: y as i32});},
        'O' => {movable_rocks.insert(Point{x: x as i32,y: y as i32});},
        _=> ()
      }
    }
  }
  let xsize = input[0].chars().collect::<Vec<char>>().len() as i32;
  let ysize = input.len() as i32;
  move_all_rocks(&movable_rocks, Point{x:0,y:-1}, &immovable_rocks, xsize, ysize).iter()
    .fold(0, |acc, rock| acc +xsize-rock.y)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut movable_rocks = HashSet::new();
  let mut immovable_rocks = HashSet::new();
  for (y,line) in input.iter().enumerate() {
    for (x,entry) in line.chars().enumerate() {
      match entry {
        '#' => {immovable_rocks.insert(Point{x: x as i32,y: y as i32});},
        'O' => {movable_rocks.insert(Point{x: x as i32,y: y as i32});},
        _=> ()
      }
    }
  }
  let mut prev_states = Vec::new();
  let xsize = input[0].chars().collect::<Vec<char>>().len() as i32;
  let ysize = input.len() as i32;
  let mut previous = movable_rocks.clone();
  prev_states.push(previous);
  for cycle in 1..CYCLES {
    println!("{}", cycle);
    movable_rocks = move_all_rocks(&movable_rocks, Point{x:0,y:-1}, &immovable_rocks, xsize, ysize);
    movable_rocks = move_all_rocks(&movable_rocks, Point{x:-1,y:0}, &immovable_rocks, xsize, ysize);
    movable_rocks = move_all_rocks(&movable_rocks, Point{x:0,y:1}, &immovable_rocks, xsize, ysize);
    movable_rocks = move_all_rocks(&movable_rocks, Point{x:1,y:0}, &immovable_rocks, xsize, ysize);
    let mut found_loop = false;
    let mut found_loop_start = 0;
    let mut loop_size = 0;
    for (i, prev_state) in prev_states.iter().enumerate() {
      println!("\t{}", i);
      if !found_loop{
        if movable_rocks == *prev_state {
          found_loop = true;
          found_loop_start = i;
          loop_size = cycle - i as i32 ;
        }
      } 
      if found_loop {
        if (CYCLES - found_loop_start as i32) % loop_size == (i as i32 - found_loop_start as i32) % loop_size {
          return prev_state.iter().fold(0, |acc, rock| acc + ysize-rock.y);
        }
      }
    }
    previous = movable_rocks.clone();
    prev_states.push(previous);
  }
  movable_rocks.iter().fold(0, |acc, rock| acc + ysize-rock.y)
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(136, solve_part1(&inp::parse_file("test_inputs/day14_test.txt")));
    assert_eq!(64, solve_part2(&inp::parse_file("test_inputs/day14_test.txt")));
  }
}