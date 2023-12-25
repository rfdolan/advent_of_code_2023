use std::vec::Vec;
use std::collections::{HashMap, HashSet};

const START: &str = "AAA";
const END: &str = "ZZZ";

fn main(){
  let vec = inp::parse_file("inputs/day08.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let mut map = HashMap::new();
  let instructions = &input[0].chars().collect::<Vec<_>>();
  for line in input[2..].into_iter() {
    let split = line.split(" = ").collect::<Vec<_>>();
    let key = split[0];
    let val = split[1];
    let buh = val.split(", ").collect::<Vec<_>>();
    let val = (&buh[0][1..], &buh[1][..buh[1].len()-1]);
    map.insert(key, val);
  }
  let mut num_steps = 0;
  let mut curr_location = START;
  let mut curr_instruction_index = 0;
  while curr_location != END {
    if curr_instruction_index >= instructions.len() {
      curr_instruction_index = 0;
    }
    match instructions[curr_instruction_index] {
      'L' => curr_location = map.get(&curr_location).unwrap().0,
      'R' => curr_location = map.get(&curr_location).unwrap().1,
      _ => ()
    }
    curr_instruction_index += 1;
    num_steps += 1;
  }
  num_steps
}

fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    return a;
  }
  gcd(b, a%b)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let mut map = HashMap::new();
  let instructions = &input[0].chars().collect::<Vec<_>>();
  let mut starting_positions = Vec::new();
  for line in input[2..].into_iter() {
    let split = line.split(" = ").collect::<Vec<_>>();
    let key = split[0];
    if key.chars().collect::<Vec<_>>()[key.len()-1] == 'A' {
      starting_positions.push(key);
    }
    let val = split[1];
    let buh = val.split(", ").collect::<Vec<_>>();
    let val = (&buh[0][1..], &buh[1][..buh[1].len()-1]);
    map.insert(key, val);
  }

  let starting_positions = starting_positions;
  let mut num_steps = 0;
  let mut curr_locations = starting_positions.clone();
  let mut curr_instruction_index = 0;
  let mut ending_positions = HashSet::new();
  let mut ending_path_lengths = Vec::new();
  while ending_path_lengths.len() != starting_positions.len() {
    num_steps += 1;
    match instructions[curr_instruction_index] {
      'L' => {curr_locations = curr_locations.clone().into_iter().map(|x| map.get(x).unwrap().0).collect::<Vec<&str>>();},
      'R' => {curr_locations = curr_locations.clone().into_iter().map(|x| map.get(x).unwrap().1).collect::<Vec<&str>>();},
      _ => ()
    }
    for location in curr_locations.clone() {
      if location.chars().collect::<Vec<_>>()[location.len()-1] == 'Z' &&  !ending_positions.contains(&location) {
        ending_positions.insert(location);
        ending_path_lengths.push(num_steps);
      }
    }
    curr_instruction_index = (curr_instruction_index + 1) % instructions.len();
  }
  let mut curr_lcm = ending_path_lengths[0];
  for i in 1..ending_path_lengths.len() {
    curr_lcm = ending_path_lengths[i] * (curr_lcm / gcd(ending_path_lengths[i], curr_lcm));
  }
  curr_lcm
}

#[cfg(test)]
mod day8_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(2, solve_part1(&inp::parse_file("test_inputs/day08_part1_test1.txt")));
    assert_eq!(6, solve_part1(&inp::parse_file("test_inputs/day08_part1_test2.txt")));
    assert_eq!(6, solve_part2(&inp::parse_file("test_inputs/day08_part2_test.txt")));
  }
}