use std::vec::Vec;
use std::ops::RangeInclusive;
use std::thread;

#[derive(Clone)]
struct Map {
  destrange: RangeInclusive<i64>,
  srcrange: RangeInclusive<i64>
}

fn get_val_from_map(target: i64, maps: &Vec<Map>) -> i64 {
  for map in maps.iter() {
    if map.srcrange.contains(&target) {
      return map.destrange.start() + (target - map.srcrange.start())
    }
  }
  target
}

fn main(){
  let vec = inp::parse_file("inputs/day05.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn parse_maps(input: &[String]) -> Vec<Vec<Map>> {
  let mut my_maps: Vec<Vec<Map>> = vec![Vec::new(); 7];
  let mut mapnum = 0;
  let mut i = 0;
  while i < input.len() {
    let line = &input[i];
    //println!("{}", line);
    if line == "" {
      mapnum+= 1;
      i+= 2;
    } else {
      let line_val = line.split(" ").map(|x| x.parse::<i64>().expect(&format!("{} invalid", x))).collect::<Vec<_>>();
      let dest = line_val[0];
      let src = line_val[1];
      let range = line_val[2];
      my_maps[mapnum].push(Map{destrange: dest..=dest+range, srcrange: src..=src+range});
      i += 1;
    }
  }
  my_maps

}
// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let seeds = &input[0].split(" ").collect::<Vec<_>>()[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
  let maps = parse_maps(&input[3..]);
  seeds.iter().fold(i64::MAX, |mut acc, seed| {acc = std::cmp::min(acc, part1_helper(*seed, &maps)); acc})
}

fn part1_helper(target: i64, maps: &Vec<Vec<Map>>) -> i64 {
  let mut seedval = target;
  for maps in maps.iter() {
    seedval = get_val_from_map(seedval, &maps);
  }
  seedval
}

fn contains_seed(seed: i64, seeds: &Vec<RangeInclusive<i64>>) -> bool {
  for range in seeds {
    if range.contains(&seed) { 
      return true;
    }
  }
  false
}

fn find_seed_from_location(location: i64, maps: &Vec<Vec<Map>>, seeds: &Vec<RangeInclusive<i64>>) -> bool {
  let mut source = location;
  for x_to_y_map in maps {
    for map in x_to_y_map {
      if map.destrange.contains(&source) {
        source = map.srcrange.start() + (source - map.destrange.start());
        break;
      }
    }
  }
  contains_seed(source, seeds)
}

fn _solve_part2_epic_version(input: &Vec<String>) -> i64 {
  let seeds = &input[0].split(" ").collect::<Vec<_>>()[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
  let mut seed_it = 0;
  let mut seedranges = Vec::new();
  while seed_it < seeds.len() {
    seedranges.push(seeds[seed_it]..=seeds[seed_it]+seeds[seed_it+1]);
    seed_it += 2;
  }

  let maps = parse_maps(&input[3..]);

  let mut handles = Vec::new();
  for range in seedranges {
    let newmaps = maps.clone();
    let handle = thread::spawn(move || {
      println!("Doing range...");
      let answer = range.fold(i64::MAX, |mut acc, seed| {acc = std::cmp::min(acc, part1_helper(seed,&newmaps)); acc});
      println!("DONE!");
      answer
    });
    handles.push(handle);
  }
  let mut min_seedval = i64::MAX;
  for handle in handles {
    min_seedval = std::cmp::min(min_seedval, handle.join().unwrap());
  }
  min_seedval
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let seeds = &input[0].split(" ").collect::<Vec<_>>()[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
  let mut seed_it = 0;
  let mut seedranges = Vec::new();
  while seed_it < seeds.len() {
    seedranges.push(seeds[seed_it]..=seeds[seed_it]+seeds[seed_it+1]);
    seed_it += 2;
  }

  let maps = parse_maps(&input[3..]);
  let maps_rev = maps.into_iter().rev().collect::<Vec<_>>();

  let mut location = 0;
  loop { // We'll find it eventually... right?
    if find_seed_from_location(location, &maps_rev, &seedranges) {
      return location;
    }
    location += 1;
  }
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    assert_eq!(35, solve_part1(&inp::parse_file("test_inputs/day05_test.txt")));
    assert_eq!(46, solve_part2(&inp::parse_file("test_inputs/day05_test.txt")));
    assert_eq!(46, _solve_part2_epic_version(&inp::parse_file("test_inputs/day05_test.txt")));
  }
}