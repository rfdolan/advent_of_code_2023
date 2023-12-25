use std::vec::Vec;

// 0: high card, 1: one pair, 2: two pair, 3: three of kind, 4: full house, 5: four of kind, 6: five of kind

const FIVE_OF_KIND_VALUE: i32 = 6;
const FOUR_OF_KIND_VALUE: i32 = 5;
const FULL_HOUSE_VALUE: i32 = 4;
const THREE_OF_KIND_VALUE: i32 = 3;
const TWO_PAIR_VALUE: i32 = 2;
const PAIR_VALUE: i32 = 1;

const CARD_VALUES: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];

#[derive(Clone, Debug)]
struct Hand {
  contents: Vec<char>,
  bid: i32,
  value: i32 // Value determied by high card, one pair, two pair, etc
}




fn main(){
  let vec = inp::parse_file("inputs/day07.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_hand_value(hand: &Vec<char>) -> i32 {
  let mut sorted = hand.clone();
  sorted.sort_by(|a, b| b.cmp(a));
  let mut index = 0;
  let mut found_pair = false;
  let mut found_three = false;
  while index < sorted.len() {
    let curr_char = sorted[index];
    let occurrences = sorted.clone().iter().filter(|x| **x == curr_char).count();
    match occurrences {
      5 => return FIVE_OF_KIND_VALUE,
      4 => return FOUR_OF_KIND_VALUE,
      3 => {
        if found_pair { return FULL_HOUSE_VALUE;} else { found_three = true; index += 3;}
      },
      2 => {
        if found_three { return FULL_HOUSE_VALUE;} else if found_pair {return TWO_PAIR_VALUE;} else {found_pair = true; index += 2;}
      },
      _ => index += 1
    }
  }
  if found_three {
    return THREE_OF_KIND_VALUE;
  }
  if found_pair {
    return PAIR_VALUE;
  }
  0
}

fn parse_hands(input: &Vec<String>) -> Vec<Hand> {
  let mut hands = Vec::new();
  for line in input.iter() {
    let split = line.split(" ").collect::<Vec<_>>();
    let contents = String::from(split[0]).chars().collect::<Vec<char>>();
    let bid = split[1].parse::<i32>().unwrap();
    let value = get_hand_value(&contents);
    hands.push(Hand{contents, bid, value});
  }
  hands
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let hands = parse_hands(input);
  let mut result: Vec<Hand> = Vec::new();
  for hand in hands {
    //println!("{:?}", hand);
    let mut index = 0;
    let mut added = false;
    'result_iter: while index < result.len() && !added {
      //println!("jkl");
      let other = result[index].clone();
      if other.value > hand.value {
        result.insert(index, hand.clone());
            added = true;
        break;
      } 
      if other.value == hand.value {
        //println!("{:?} == {:?}", other.contents, hand.contents);
        let mut contents_index = 0;
        while contents_index < hand.contents.len() {
          let otherchar = other.contents[contents_index];
          let handchar = hand.contents[contents_index];
          let otherchar_value = CARD_VALUES.iter().position(|&x| x == otherchar).unwrap();
          let handchar_value = CARD_VALUES.iter().position(|&x| x == handchar).unwrap();
          if otherchar_value > handchar_value {
            result.insert(index, hand.clone());
            added = true;
            break;
          }
          if handchar_value > otherchar_value {
            //println!("{} < {}: added = {}, index = {}", otherchar_value, handchar_value, added, index);
            break 'result_iter;
          }
          contents_index += 1;
        }
      } else {
        index += 1;
      }
    }
    if !added {
      result.push(hand);
    }

  }
  println!("{:?}", result);
  result.iter().enumerate().fold(0, |acc, (i,x)| acc + ((i as i32+1) * x.bid))
}

// Solution for part 2
fn solve_part2(_input: &Vec<String>) -> i32 {
  0
}

#[cfg(test)]
mod day7_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(6440, solve_part1(&inp::parse_file("test_inputs/day07_test.txt")));
  }
}