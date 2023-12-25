use std::vec::Vec;

struct Card {
  winning_nums: Vec<i32>,
  my_nums: Vec<i32>
}

fn main(){
  let vec = inp::parse_file("inputs/day04.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn string_to_list_of_ints(input: &str) -> Vec<i32> {
  input.split(" ").filter_map(|x| match x.parse::<i32>() {
    Ok(x)=>  Some(x),
    _ => None
  }).collect::<Vec<i32>>()
}

fn parse_input(input: &Vec<String>) -> Vec<Card> {
  let mut cards: Vec<Card> = Vec::new();
  for line in input.iter() {
    let lists = &line.split(":").collect::<Vec<_>>()[1..];
    let lists = lists.iter().fold(String::from(""), |mut acc, string| {acc.push_str(string); acc});
    let lists = lists.split("|").collect::<Vec<_>>();
    let winning_nums = string_to_list_of_ints(lists[0]);
    let my_nums = string_to_list_of_ints(lists[1]);
    cards.push(Card{winning_nums, my_nums});
  }
  cards
}

fn get_num_matching(card: &Card) -> u32 {
  let mut card_total = 0;
  for winning_number in card.winning_nums.iter() {
    if card.my_nums.contains(&winning_number) {
      card_total += 1;
    }
  }
  card_total
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i32 {
  let mut total = 0;
  let cards = parse_input(input);
  for card in cards {
    let card_total = get_num_matching(&card);
    if card_total > 0 {
      total += (2 as i32).pow(card_total -1);
    }
  }
  total
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i32 {
  let cards = parse_input(&input);
  let mut num_each_card: Vec<i32>= vec![1; cards.len()];
  let mut total = 0;
  for (card_num, card) in cards.iter().enumerate() {
    let num_card_owned = num_each_card[card_num];
    total += num_card_owned;
    let card_value = get_num_matching(card);
    for card_num_to_add_to in  card_num+1..=(card_num+(card_value as usize)) {
      num_each_card[card_num_to_add_to as usize] += num_card_owned;
    }
  }
  total
}

#[cfg(test)]
mod day4_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(13, solve_part1(&inp::parse_file("test_inputs/day04_test.txt")));
    assert_eq!(30, solve_part2(&inp::parse_file("test_inputs/day04_test.txt")));
  }
}