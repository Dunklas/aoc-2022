use std::collections::HashSet;

use crate::utils::coordinate::Coordinate;

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut head = Coordinate::new(0, 0);
    let mut visited: Vec<_> = vec![Coordinate::new(0, 0)];
    parse(input).into_iter().for_each(|(dir, n)| {
        (0..n).for_each(|_| {
            head = move_head(&head, dir);
            visited.push(follow(&head, visited.last().unwrap()));
        });
    });
    visited.into_iter().collect::<HashSet<Coordinate>>().len()
}

fn part2(input: &str) -> usize {
    let mut knots = (0..10).map(|_| Coordinate::new(0, 0)).collect::<Vec<_>>();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    parse(input).into_iter().for_each(|(dir, n)| {
        (0..n).for_each(|_| {
            knots[0] = move_head(&knots[0], dir);
            (1..knots.len()).for_each(|i| {
                knots[i] = follow(&knots[i - 1], &knots[i]);
            });
            visited.insert(knots.last().unwrap().clone());
        });
    });
    visited.len()
}

fn move_head(head: &Coordinate, dir: &str) -> Coordinate {
    match dir {
        "R" => Coordinate::new(head.x + 1, head.y),
        "D" => Coordinate::new(head.x, head.y - 1),
        "L" => Coordinate::new(head.x - 1, head.y),
        "U" => Coordinate::new(head.x, head.y + 1),
        _ => panic!("Unexpected dir: {}", dir),
    }
}

fn follow(head: &Coordinate, tail: &Coordinate) -> Coordinate {
    if tail == head || tail.is_adjacent_to(head, true) {
        return Coordinate::new(tail.x, tail.y);
    }
    Coordinate::new(
        tail.x + (head.x - tail.x).signum() * 1,
        tail.y + (head.y - tail.y).signum() * 1,
    )
}

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|parts| (parts[0], parts[1].parse::<i32>().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(1, part2(input));
    }

    #[test]
    fn test_part2_larger() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, part2(input));
    }
}
