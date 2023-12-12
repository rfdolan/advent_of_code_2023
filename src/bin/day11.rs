use std::vec::Vec;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
struct Point {
  col: i64,
  row:i64
}

fn main(){
  let vec = inp::parse_file("inputs/day11.txt");
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&vec));
  println!("Part 2: {}", solve_part2(&vec));
}

fn solve(galaxies: &Vec<Point>, occupied_rows: &HashSet<i64>, occupied_cols: &HashSet<i64>, galaxy_expansion_factor: i64) -> i64 {
  let mut total = 0;
  for first in 0..galaxies.len(){
    for second in first+1..galaxies.len() {
      let galaxy = &galaxies[first];
      let other = &galaxies[second];
      if other == galaxy {
        continue;
      }
      let mut empty_cols = 0;
      for col in std::cmp::min(galaxy.col, other.col)..std::cmp::max(galaxy.col, other.col) {
        if !occupied_cols.contains(&col) {
          empty_cols += 1;
        }
      }
      let col_dist = (std::cmp::max(galaxy.col, other.col)-std::cmp::min(galaxy.col, other.col)) + (galaxy_expansion_factor * empty_cols) - empty_cols;
      let mut empty_rows = 0;
      for row in std::cmp::min(galaxy.row, other.row)..std::cmp::max(galaxy.row, other.row) {
        if !occupied_rows.contains(&row) {
          empty_rows += 1;
        }
      }
      let row_dist = (std::cmp::max(galaxy.row, other.row)-std::cmp::min(galaxy.row, other.row)) + (galaxy_expansion_factor * empty_rows) - empty_rows;
      total += row_dist + col_dist;
    }
  }

  total

}
fn parse_galaxies(input: &Vec<String>) -> (Vec<Point>, HashSet<i64>, HashSet<i64>) {
  let mut galaxies = Vec::new();
  let mut occupied_rows = HashSet::new();
  let mut occupied_cols = HashSet::new();
  for (row, line) in input.iter().enumerate() {
    for (col, space) in line.chars().enumerate() {
      if space == '#' {
        galaxies.push(Point{col: col as i64, row: row as i64});
        occupied_rows.insert(row as i64);
        occupied_cols.insert(col as i64);
      }
    }
  }
  (galaxies, occupied_rows, occupied_cols)
}

// Solution for part 1
fn solve_part1(input: &Vec<String>) -> i64 {
  let (galaxies, occupied_rows, occupied_cols) = parse_galaxies(input);
  solve(&galaxies, &occupied_rows, &occupied_cols, 2)
}

// Solution for part 2
fn solve_part2(input: &Vec<String>) -> i64 {
  let (galaxies, occupied_rows, occupied_cols) = parse_galaxies(input);
  solve(&galaxies, &occupied_rows, &occupied_cols, 1000000)
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    let (galaxies, occupied_rows, occupies_cols) = parse_galaxies(&inp::parse_file("test_inputs/day11_test.txt"));
    assert_eq!(374, solve(&galaxies, &occupied_rows, &occupies_cols, 2));
    assert_eq!(1030, solve(&galaxies, &occupied_rows, &occupies_cols, 10));
    assert_eq!(8410, solve(&galaxies, &occupied_rows, &occupies_cols, 100));
  }
}