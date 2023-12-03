use std::vec::Vec;

struct Point {
  x: i32,
  y: i32
}

struct Number {
  start : Point,
  width : i32,
  value: i32
}

struct Component {
  symbol: char,
  position: Point
}

fn is_point_around_other(point1: &Point, point2: &Point) -> bool {
  (point1.x - point2.x).abs() <= 1 && (point1.y - point2.y).abs() <= 1
}

fn is_num_around_component(part_location: &Point, number: &Number) -> bool {
  for numx in number.start.x..(number.start.x+number.width as i32) {
    if is_point_around_other(&part_location, &Point{x: numx, y: number.start.y}) {
      //println!("Found vlue {} at point {}, {} around {}, {}", number.value, numx, number.start.y, part_location.x, part_location.y);
      return true;
    }
  }
  false
}

fn parse_input(input: &Vec<String>) -> (Vec<Number>, Vec<Component>) {
  let mut numbers: Vec<Number> = Vec::new();
  let mut components: Vec<Component> = Vec::new();
  for (y, line) in input.iter().enumerate() {
    let mut curr_num = 0;
    let mut curr_width = 0;
    for (x, character) in line.chars().into_iter().enumerate() {
      match character {
        i if i.is_numeric() =>{
          let i = i.to_digit(10).unwrap();
          curr_num = (curr_num * 10) + i as i32;
          curr_width += 1;
        },
        i => {
          if curr_num != 0 {
            let num = Number{start: Point{x: x as i32-curr_width, y: y as i32}, width: curr_width, value: curr_num};
            numbers.push(num);
            curr_num = 0;
            curr_width = 0;
          }
          match i {
            '.' => (),
            i => {
              components.push(Component{symbol: i, position: Point{x: x as i32,y: y as i32}});
            }
          }
        }
      }
    }
    if curr_num != 0 {
      let num = Number{start: Point{x: line.len() as i32-curr_width, y: y as i32}, width: curr_width, value: curr_num};
      numbers.push(num);
    }
  }
  (numbers, components)

}
fn main(){
  let vec = inp::parse_file("inputs/day03.txt");

  let (numbers, components) = parse_input(&vec);
  // Put the code to do the thing here
  println!("Part 1: {}", solve_part1(&numbers, &components));
  println!("Part 2; {}", solve_part2(&numbers, &components));
}

// Solution for part 1
fn solve_part1(numbers: &Vec<Number>, components: &Vec<Component>) -> i32 {
  let mut total = 0;
  for number in numbers.iter() {
    for component in components {
      if is_num_around_component(&component.position, number) {
        total += number.value;
        break;
      }
    }
  }
  total
}

// Solution for part 2
fn solve_part2(numbers: &Vec<Number>, components: &Vec<Component>) -> i32 {
  let mut total = 0;
  for component in components.iter() {
    if component.symbol == '*' {
      let mut adjacent_nums = 0;
      let mut gear_ratio = 1;
      for number in numbers.iter() {
        if is_num_around_component(&component.position, number) {
          adjacent_nums += 1;
          gear_ratio *= number.value;
        }
      }
      if adjacent_nums == 2 {
        total += gear_ratio;
      }
    }
  }
  total
}

#[cfg(test)]
mod day3_tests {
  use super::*;
  #[test]
  fn test() {
    let (numbers, components) = parse_input(&inp::parse_file("test_inputs/day03_part1_test.txt"));
    let (numbers_alt, components_alt) = parse_input(&inp::parse_file("test_inputs/day03_part1_test2.txt"));
    assert_eq!(4361, solve_part1(&numbers, &components));
    assert_eq!(12, solve_part1(&numbers_alt, &components_alt));
    assert_eq!(467835, solve_part2(&numbers, &components));
  }
}