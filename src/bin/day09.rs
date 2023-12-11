use std::vec::Vec;

fn main(){
  let vec = inp::parse_file("inputs/day09.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn check_multiple(nums: &Vec<i32>) -> Vec<i32> {
  let mut i = 1;
  let mut ret = Vec::new();
  while i < nums.len() {
    ret.push(nums[i] - nums[i-1]);
    i+=1;
  }
  ret
}

fn has_all_zeroes(nums: &Vec<i32>) ->bool {
  for num in nums {
    if *num != 0 { return false;}
  }
  true
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut grand_total = 0;
  for line in input {
    let mut nums = line.split(" ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut final_nums = Vec::new();
    final_nums.push(nums[nums.len()-1]);
    while !has_all_zeroes(&nums) {
      nums = check_multiple(&nums);
      final_nums.push(nums[nums.len()-1]);
    }
    final_nums.reverse();
    let mut last = 0;
    for n in final_nums {
      let next_num = n + last;
      last = next_num;
    }
    grand_total += last;
  }
  grand_total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut grand_total = 0;
  for line in input {
    let mut nums = line.split(" ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut final_nums = Vec::new();
    final_nums.push(nums[0]);
    while !has_all_zeroes(&nums) {
      nums = check_multiple(&nums);
      final_nums.push(nums[0]);
    }
    final_nums.reverse();
    let mut last = 0;
    for n in final_nums {
      let next_num = n + last;
      last = next_num;
    }
    grand_total += last;
  }
  grand_total
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(114, solve_part1(&inp::parse_file("test_inputs/day09_test.txt")));
    assert_eq!(2, solve_part2(&inp::parse_file("test_inputs/day09_test.txt")));
  }
}