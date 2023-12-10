use std::vec::Vec;
use std::collections::{HashMap, HashSet, VecDeque};

const SURROUNDING: [Point; 4] = [Point{x:0,y:1}, Point{x:0, y:-1}, Point{x: 1, y:0}, Point{x: -1, y:0}];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32
}

fn add_points(point: Point, other: Point) -> Point {
  Point{x: point.x + other.x, y: point.y + other.y}
}

fn search_surrounding(point: Point, pipes: &HashMap<Point, (Point, Point)>) -> (Point, Point) {
  let mut return_points = Vec::new();
  for surrounding in SURROUNDING {
    let to_check = add_points(point, surrounding);
    match pipes.get(&to_check) {
      Some(x) => {
        if x.0 == point || x.1 == point {
          return_points.push(to_check);
        }
      },
      None => ()
    }
  }
  if return_points.len() >= 2 {
    return (return_points[0], return_points[1]);
  }
  (Point{x:0,y:0},Point{x:0,y:0})

}

fn parse_pipes(input: &Vec<String>) -> (HashMap<Point, (Point, Point)>, Point){
  let mut start = Point{x:0,y:0};
  let mut pipes: HashMap<Point, (Point, Point)> = HashMap::new();
  for (y,line) in input.iter().enumerate() {
    for (x,pipe_char) in line.chars().enumerate() {
      let currpos = Point{x: x as i32, y: y as i32};
      match pipe_char {
        '|' => {pipes.insert(currpos,(Point{x: x as i32, y : y as i32 - 1}, Point{x: x as i32, y: y as i32 + 1}));},
        '-' => {pipes.insert(currpos, (Point{x: x as i32 - 1,y: y as i32},Point{x: x as i32 + 1, y: y as i32}));},
        'L' => {pipes.insert(currpos, (Point{x: x as i32,y: y as i32 - 1},Point{x: x as i32 + 1, y: y as i32}));},
        'F' => {pipes.insert(currpos,(Point{x: x as i32,y: y as i32 + 1},Point{x: x as i32 + 1, y: y as i32}));},
        '7' => {pipes.insert(currpos, (Point{x: x as i32 - 1,y: y as i32},Point{x: x as i32, y: y as i32 + 1}));},
        'J' => {pipes.insert(currpos, (Point{x: x as i32 - 1,y: y as i32},Point{x: x as i32, y: y as i32 - 1}));},
        'S' => {
          start = Point{x: x as i32, y: y as i32};
        },
        _ => (),
      }
    }
  }
  pipes.insert(start, search_surrounding(start, &pipes));
  (pipes, start)
}

fn main(){
  let vec = inp::parse_file("inputs/day10.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let (pipes, start) = parse_pipes(input);
  let start_pipe = pipes.get(&start).unwrap();
  let mut visited = HashSet::new();
  let mut curr_pos = start_pipe.0;
  visited.insert(start);
  let mut len = 1;

  while !visited.contains(&curr_pos) {
    visited.insert(curr_pos);
      len += 1;
    let curr_target = pipes.get(&curr_pos).unwrap();
    if !visited.contains(&curr_target.0) {
      curr_pos = curr_target.0;
    } else if !visited.contains(&curr_target.1) {
      curr_pos = curr_target.1;
    }
  }
  len/2 + 1
}


fn escape(start: Point, pipes: &HashMap<Point, (Point, Point)>, gridsize: Point) -> bool {
  // do dfs here

  false
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let grid_size = Point{y:input.len() as i32, x: input[0].chars().collect::<Vec<char>>().len() as i32};
  let (pipes, start) = parse_pipes(input);
  let mut captured_spaces = 0;
  for y in 0..grid_size.y {
    for x in 0..grid_size.x {
      if !escape(Point{x: x as i32, y: y as i32}, &pipes, grid_size) {
        captured_spaces += 1;
      }
    }
  }
  captured_spaces

}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(4, solve_part1(&inp::parse_file("test_inputs/day10_part1_test.txt")));
    assert_eq!(8, solve_part1(&inp::parse_file("test_inputs/day10_part1_test2.txt")));
    assert_eq!(4, solve_part1(&inp::parse_file("test_inputs/day10_part2_test.txt")));
    assert_eq!(10, solve_part1(&inp::parse_file("test_inputs/day10_part2_test2.txt")));
  }
}