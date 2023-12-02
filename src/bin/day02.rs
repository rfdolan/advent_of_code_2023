use std::vec::Vec;

const PART1_REDMAX: i32 = 12;
const PART1_GREENMAX: i32 = 13;
const PART1_BLUEMAX: i32 = 14;

// RGB
#[derive (Copy, Clone)]
struct Draw(i32, i32, i32);

fn main(){
  let vec = inp::parse_file("inputs/day02.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn parse_game(input: &String) -> Vec<Draw> {
  let s = &input.split(" ").collect::<Vec<_>>()[2..];
  let s = s.iter().fold(String::from(""), |mut acc, string| {acc.push_str(string); acc});
  let unparsed_draws = s.split(";").collect::<Vec<_>>();
  let mut draws: Vec<Draw> = Vec::new();
  for draw in unparsed_draws {
    let single_pull = draw.split(",");
    let mut this_draw = Draw(0,0,0);
    for pull in single_pull {
      let num = pull.chars().into_iter().filter(|x| x.is_numeric()).into_iter().collect::<String>().parse::<i32>().unwrap();
      let word = pull.chars().into_iter().filter(|x| !x.is_numeric()).into_iter().collect::<String>();
      match word.as_str() {
        "red" => this_draw.0 = num,
        "green" => this_draw.1 = num,
        "blue" => this_draw.2 = num,
        x => println!("{x} isn't a color")
      }
    }
    draws.push(this_draw);
  }
  draws


}
// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut total = 0;
  for (x,line) in input.iter().enumerate() {
    let game = parse_game(&line);
    let mut gameval: i32 = x as i32+1;
    for draw in game {
      if draw.0 > PART1_REDMAX ||
      draw.1 > PART1_GREENMAX ||
      draw.2 > PART1_BLUEMAX  {
        gameval = 0;
      }
    }
    total += gameval;
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut total = 0;
  for line in input.iter(){
    let game = parse_game(&line);
    let mut minred = 0;
    let mut mingreen = 0;
    let mut minblue = 0;
    for draw in game{
      minred = std::cmp::max(minred, draw.0);
      mingreen = std::cmp::max(mingreen, draw.1);
      minblue = std::cmp::max(minblue, draw.2);
    }
    total += minred * mingreen * minblue;
  }
  total
}

#[cfg(test)]
mod day2_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(8, solve_part1(&inp::parse_file("test_inputs/day02_test.txt")));
    assert_eq!(2286, solve_part2(&inp::parse_file("test_inputs/day02_test.txt")));
  }
}