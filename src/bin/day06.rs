use std::vec::Vec;
use std::iter::zip;

fn main(){
  let vec = inp::parse_file("inputs/day06.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn do_race(time_held: i64, total_time: i64) -> i64 {
  (total_time - time_held) * time_held
}

fn find_solutions(time: i64, dist_target: i64) -> i64 {
  for possible_time in 0..=time {
    if do_race(possible_time, time) > dist_target {
      return time - (possible_time * 2) + 1;
    }
  }
  0
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let times= input[0].split(" ").filter_map(|x| match x.parse::<i64>() {
    Ok(x) => Some(x),
    _ => None
  }).collect::<Vec<_>>();
  let dists = input[1].split(" ").filter_map(|x| match x.parse::<i64>() {
    Ok(x) => Some(x),
    _ => None
  }).collect::<Vec<_>>();

  let races = zip(times, dists).collect::<Vec<_>>();
  let mut total = 1;
  for race in races {
    total *= find_solutions(race.0, race.1);
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  // stupid stupid stupid parsing
  let time = input[0].chars().filter(|x| !x.is_whitespace() && x.is_numeric()).collect::<String>().parse::<i64>().unwrap();
  let dist = input[1].chars().filter(|x| !x.is_whitespace() && x.is_numeric()).collect::<String>().parse::<i64>().unwrap();
  find_solutions(time, dist)
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(288, solve_part1(&inp::parse_file("test_inputs/day06_test.txt")));
    assert_eq!(71503, solve_part2(&inp::parse_file("test_inputs/day06_test.txt")));
  }
}