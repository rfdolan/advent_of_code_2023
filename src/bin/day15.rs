use std::vec::Vec;

const NUM_HASHABLE: i32 = 256;
const MAGIC_HASH_NUMBER: i32 = 17;
#[derive(Clone)]
struct Lens {
  focal_length: i32,
  label: String
}

impl PartialEq for Lens {
  fn eq(&self, other: &Self) -> bool {
    self.label == other.label
  }
}
impl Eq for Lens {}



fn main(){
  let vec = inp::parse_file("inputs/day15.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn do_hash_char(c: char, curr_val: i32) -> i32 {
  let mut ret = curr_val + c as i32 ;
  ret *= MAGIC_HASH_NUMBER;
  ret % NUM_HASHABLE
}

fn do_hash(string: &str) -> i32 {
  let mut total = 0;
  for c in string.chars() {
    total = do_hash_char(c, total);
  }
  total
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut total = 0;
  for string in input[0].split(",") {
    total += do_hash(&string);
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let mut boxes =Vec::new();
  for _ in 0..NUM_HASHABLE {
    let new_box: Vec<Lens> = Vec::new();
    boxes.push(new_box);
  }
  for string in input[0].split(",") {
    // Remove lens
    if string.contains('-') {
      let split = string.split("-").collect::<Vec<_>>();
      let label = split[0].to_string();
      let target_box = do_hash(&label);
      if let Some(my_box) = boxes.get(target_box as usize) {
        let mut new_box = my_box.clone();
        if let Some(index) = my_box.iter().position(|r| r.label==label) {
          new_box.remove(index);
        }
        boxes[target_box as usize] = new_box;
      }
    }
    // Add lens
    if string.contains('=') {
      let split = string.split("=").collect::<Vec<_>>();
      let label = split[0].to_string();
      let focal_length = split[1].parse::<i32>().unwrap();
      let target_box = do_hash(&label);
      if let Some(my_box) = boxes.get(target_box as usize) {
        let mut new_box = my_box.clone();
        if let Some(index) = my_box.iter().position(|r| r.label==label) {
          new_box[index] = Lens{label, focal_length};
        } else {
          new_box.push(Lens{label, focal_length});
        }
        boxes[target_box as usize] = new_box;
      }
    }
  }
  let mut total = 0;
  for (box_num, curr_box) in boxes.iter().enumerate() {
    for (lens_num, lens) in curr_box.iter().enumerate() {
      total += (1+box_num as i32)*(lens_num + 1) as i32*lens.focal_length;
    }
  }
  total
}

#[cfg(test)]
mod day15 {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(1320, solve_part1(&inp::parse_file("test_inputs/day15_test.txt")));
    assert_eq!(145, solve_part2(&inp::parse_file("test_inputs/day15_test.txt")));
  }
}