use std::vec::Vec;
use std::cmp::Ordering;

// 0: high card, 1: one pair, 2: two pair, 3: three of kind, 4: full house, 5: four of kind, 6: five of kind

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandTypes {
  HighCard = 0,
  Pair = 1,
  TwoPair = 2,
  ThreeOfKind = 3,
  FullHouse = 4,
  FourOfKind = 5,
  FiveOfKind = 6
}
const CARD_VALUES: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
const JOKER_CARD_VALUES: [char; 13] = ['J','2','3','4','5','6','7','8','9','T','Q','K','A'];

#[derive(Clone, Debug, Eq)]
struct Hand {
  contents: Vec<char>,
  bid: i32,
  hand_type: HandTypes,
  use_joker: bool
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.hand_type.cmp(&other.hand_type) {
      Ordering::Equal => {
        let card_values = if self.use_joker {JOKER_CARD_VALUES} else {CARD_VALUES};
        for i in 0..self.contents.len() {
          let self_card = self.contents[i];
          let other_card = other.contents[i];
          for  card in card_values.iter() {
            if *card == self_card && *card != other_card {
              return Ordering::Less;
            }
            if *card != self_card && *card == other_card {
              return Ordering::Greater;
            }
          }
        }
        Ordering::Equal
      },
      x => x
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.hand_type == other.hand_type
  }
}

fn main(){
  let vec = inp::parse_file("inputs/day07.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn get_hand_value(hand: &Vec<char>) -> HandTypes {
  let mut sorted = hand.clone();
  sorted.sort_by(|a, b| b.cmp(a));
  let mut index = 0;
  let mut found_pair = false;
  let mut found_three = false;
  while index < sorted.len() {
    let curr_char = sorted[index];
    let occurrences = sorted.clone().iter().filter(|x| **x == curr_char).count();
    match occurrences {
      5 => return HandTypes::FiveOfKind,
      4 => return HandTypes::FourOfKind,
      3 => {
        if found_pair { return HandTypes::FullHouse;} else { found_three = true; index += 3;}
      },
      2 => {
        if found_three { return HandTypes::FullHouse;} else if found_pair {return HandTypes::TwoPair;} else {found_pair = true; index += 2;}
      },
      _ => index += 1
    }
  }
  if found_three {
    return HandTypes::ThreeOfKind;
  }
  if found_pair {
    return HandTypes::Pair;
  }
  HandTypes::HighCard
}

fn parse_hands(input: &Vec<String>) -> Vec<Hand> {
  let mut hands = Vec::new();
  for line in input.iter() {
    let split = line.split(" ").collect::<Vec<_>>();
    let contents = String::from(split[0]).chars().collect::<Vec<char>>();
    let bid = split[1].parse::<i32>().unwrap();
    let hand_type = get_hand_value(&contents);
    hands.push(Hand{contents, bid, hand_type, use_joker : false});
  }
  hands
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut hands = parse_hands(input);
  hands.sort();
  hands.iter().enumerate().fold(0, |acc, (i,x)| acc + ((i as i32+1) * x.bid))
}

fn get_joker_value(hand: &Hand) -> HandTypes {
  let num_jokers = hand.contents.iter().filter(|&x| *x == 'J').count();
  match hand.hand_type {
    HandTypes::HighCard => if num_jokers == 1 {HandTypes::Pair} else {HandTypes::HighCard},
    HandTypes::Pair => if num_jokers == 1 {HandTypes::ThreeOfKind} else if num_jokers == 2 {HandTypes::ThreeOfKind} else {HandTypes::Pair},
    HandTypes::TwoPair => if num_jokers == 1 {HandTypes::FullHouse} else if num_jokers == 2 {HandTypes::FourOfKind} else {HandTypes::TwoPair},
    HandTypes::ThreeOfKind => if num_jokers > 0 {HandTypes::FourOfKind} else {HandTypes::ThreeOfKind},
    HandTypes::FullHouse => if num_jokers > 0 {HandTypes::FiveOfKind} else {HandTypes::FullHouse},
    HandTypes::FourOfKind => if num_jokers > 0 {HandTypes::FiveOfKind} else {HandTypes::FourOfKind},
    HandTypes::FiveOfKind => HandTypes::FiveOfKind
  }
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let hands = parse_hands(input);
  let mut joker_hands = hands.into_iter().map(|x| Hand{contents: x.contents.clone(), bid: x.bid, hand_type: get_joker_value(&x), use_joker: true}).collect::<Vec<Hand>>();
  joker_hands.sort();
  joker_hands.iter().enumerate().fold(0, |acc, (i,x)| acc + ((i as i32+1) * x.bid))
}

#[cfg(test)]
mod day7_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(6440, solve_part1(&inp::parse_file("test_inputs/day07_test.txt")));
    assert_eq!(5905, solve_part2(&inp::parse_file("test_inputs/day07_test.txt")));
  }
}