use std::{cmp, collections::HashSet, iter};
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::coordinate::Coordinate;

lazy_static! {
    static ref NUMBER_PAIRS: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
}

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut obstacles = parse(input);
    let rock_count = obstacles.len();
    let y_max = obstacles.iter().map(|c| c.y).max().unwrap();
    simulate_flow(&mut obstacles, y_max, false);
    obstacles.len() - rock_count
}

fn part2(input: &str) -> usize {
    let mut obstacles = parse(input);
    let rock_count = obstacles.len();
    let y_max = obstacles.iter().map(|c| c.y).max().unwrap();
    simulate_flow(&mut obstacles, y_max + 2 - 1, true);
    obstacles.len() - rock_count
}

fn simulate_flow(obstacles: &mut HashSet<Coordinate>, y_max: i32, solid_floor: bool) {
    let mut flow = iter::repeat(Coordinate::new(500, 0));
    while let Some(sand) = flow.next() {
        if obstacles.contains(&sand) {
            return;
        }
        let new_pos = fall(sand, &obstacles, y_max);
        if !solid_floor && new_pos.y == y_max {
            return;
        }
        obstacles.insert(new_pos);
    }
}

fn fall(sand: Coordinate, obstacles: &HashSet<Coordinate>, y_max: i32) -> Coordinate {
    for dx in vec![0, -1, 1] {
        let to_test = Coordinate::new(sand.x + dx, sand.y + 1);
        if !obstacles.contains(&to_test) && to_test.y <= y_max {
            return fall(to_test, obstacles, y_max);
        }
    }
    sand
}

fn parse(input: &str) -> HashSet<Coordinate> {
    input.lines()
        .map(|line| parse_obstacle(line))
        .flatten()
        .collect()
}

fn parse_obstacle(line: &str) -> HashSet<Coordinate> {
    let mut rock: HashSet<Coordinate> = HashSet::new();
    let all_pairs = NUMBER_PAIRS.captures_iter(line)
        .map(|cap| (cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    let mut all_pairs = all_pairs.windows(2);
    while let Some([left, right]) = all_pairs.next() {
        let x_range: Vec<_> = match left.0 == right.0 {
            true => vec![left.0; (left.1.abs_diff(right.1) + 1) as usize].into_iter().collect(),
            false => (cmp::min(left.0, right.0)..cmp::max(left.0, right.0) + 1).collect()
        };
        let y_range: Vec<_> = match left.1 == right.1 {
            true => vec![left.1; (left.0.abs_diff(right.0) + 1) as usize].into_iter().collect(),
            false => (cmp::min(left.1, right.1)..cmp::max(left.1, right.1) + 1).collect()
        };
        x_range.into_iter().zip(y_range.into_iter()).for_each(|(x, y)| {
           rock.insert(Coordinate::new(x, y));
        })
    }
    rock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part2(input), 93);
    }
}
