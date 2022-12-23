use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::coordinate::Coordinate;

lazy_static! {
    static ref COORDINATES: Regex =
        Regex::new(r"x=(\-?\d+), y=(\-?\d+).*x=(\-?\d+), y=(\-?\d+)$").unwrap();
}

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input, 2000000));
    println!("Part 2: {}", part2(input, 4000000).unwrap());
}

fn part1(input: &str, target_y: i32) -> u32 {
    let sensors = parse(input);
    let y_coverage = row_coverage(&sensors, target_y, None);
    let y_coverage = merge_ranges(y_coverage);
    y_coverage.iter().map(|r| r.start.abs_diff(r.end) - 1).sum()
}

fn part2(input: &str, max: i32) -> Option<i64> {
    let sensors = parse(input);
    for y in 0i32..max {
        let y_coverage = row_coverage(&sensors, y, Some(max));
        let y_coverage = merge_ranges(y_coverage);
        if y_coverage.len() == 1 {
            continue;
        }
        return Some(y_coverage.first().unwrap().end as i64 * 4000000 + y as i64);
    }
    None
}

fn merge_ranges(mut ranges: Vec<Range<i32>>) -> Vec<Range<i32>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    ranges
        .into_iter()
        .fold(Vec::<Range<i32>>::new(), |mut acc, range| {
            match acc.pop() {
                Some(prev_range) => {
                    if prev_range.end >= range.start {
                        acc.push(prev_range.start..prev_range.end.max(range.end));
                    } else {
                        acc.push(prev_range.start..prev_range.end);
                        acc.push(range.start..range.end);
                    }
                }
                None => {
                    acc.push(range.start..range.end);
                }
            }
            acc
        })
}

fn row_coverage(sensors: &Vec<Sensor>, target_y: i32, x_cap: Option<i32>) -> Vec<Range<i32>> {
    let mut covered: Vec<Range<i32>> = Vec::new();
    for sensor in sensors {
        let centre_distance = sensor.pos.y.abs_diff(target_y);
        let current_distance = match sensor.distance.checked_sub(centre_distance) {
            Some(d) => d,
            None => continue,
        };
        covered.push(
            (sensor.pos.x - current_distance as i32)..match x_cap {
                Some(cap) => cap.min(sensor.pos.x + current_distance as i32) + 1,
                None => sensor.pos.x + current_distance as i32 + 1,
            },
        );
    }
    covered
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let cap = COORDINATES.captures(line).unwrap();
            Sensor::new(
                Coordinate::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                &Coordinate::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            )
        })
        .collect()
}

fn manhattan(c1: &Coordinate, c2: &Coordinate) -> u32 {
    (c1.x - c2.x).abs() as u32 + (c1.y - c2.y).abs() as u32
}

#[derive(Debug)]
struct Sensor {
    pos: Coordinate,
    distance: u32,
}

impl Sensor {
    pub fn new(pos: Coordinate, nearest: &Coordinate) -> Sensor {
        let distance = manhattan(&pos, nearest);
        Sensor { pos, distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input(), 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input(), 20), Some(56000011));
    }
}
