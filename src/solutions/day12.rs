use std::collections::{HashSet, VecDeque};

use crate::utils::{coordinate::Coordinate, grid::Grid};

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.find_first('E').unwrap();
    traverse(start, 'S', &grid).unwrap()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.find_first('E').unwrap();
    traverse(start, 'a', &grid).unwrap()
}

fn traverse<'a>(start: &'a Coordinate, target: char, grid: &'a Grid<char>) -> Option<usize> {
    let mut to_visit: VecDeque<(&Coordinate, usize)> = vec![(start, 0)].into_iter().collect();
    let mut visited: HashSet<&Coordinate> = HashSet::new();
    while let Some((current, len)) = to_visit.pop_front() {
        if visited.contains(current) {
            continue;
        }
        if *grid.value_at(current).unwrap() == target {
            return Some(len);
        }
        visited.insert(current);
        for neighbour in grid.adjacent(current) {
            if height(grid.value_at(current)) as i8 - height(grid.value_at(neighbour)) as i8 > 1 {
                continue;
            }
            to_visit.push_back((neighbour, len + 1));
        }
    }
    None
}

fn height(value: Option<&char>) -> char {
    match value.unwrap() {
        'S' => 'a',
        'E' => 'z',
        _ => *value.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(part1(input), 31);
    }

    #[test]
    fn test_part2() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(part2(input), 29);
    }
}
