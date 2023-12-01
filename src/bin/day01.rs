use std::vec::Vec;

const NUMBERS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main(){
  let vec = inp::parse_file("day01.txt");
  solve(&vec);
}

fn solve_part1(line: &String) -> u32 {
  let chars = line.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>();
  let firstnum = chars[0];
  let lastnum = chars[chars.len() - 1];
  let firstnum = firstnum.to_digit(10).unwrap();
  let lastnum = lastnum.to_digit(10).unwrap();
  (firstnum*10) + lastnum
}

fn solve_part2(line: &String) -> u32 {
  let mut mutline = line.clone();
  for (i, number) in NUMBERS.iter().enumerate(){
    let charlist = number.chars().collect::<Vec<_>>();
    let firstletter = charlist[0].to_string();
    let lastletter = charlist[charlist.len() - 1].to_string();
    mutline = mutline.replace(number, (firstletter + &(i+1).to_string() + &lastletter).as_str());
  }
  solve_part1(&mutline)
}

// Solution
fn solve(input: &Vec<String>) {
  println!("Part 1: {}", input.iter().fold(0, |acc, line| acc + solve_part1(line)));
  println!("Part 2: {}", input.iter().fold(0, |acc, line| acc + solve_part2(line)));
}