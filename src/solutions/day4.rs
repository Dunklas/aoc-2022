use std::collections::HashSet;

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split(',').map(to_range).collect::<Vec<_>>())
        .filter(|x| overlaps(x, true))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split(',').map(to_range).collect::<Vec<_>>())
        .filter(|x| overlaps(x, false))
        .count()
}

fn to_range(raw_range: &str) -> HashSet<u32> {
    let parts = raw_range.split('-').collect::<Vec<&str>>();
    let start = parts[0].parse::<u32>().unwrap();
    let end = parts[1].parse::<u32>().unwrap();
    (start..end + 1).collect()
}

fn overlaps(ranges: &[HashSet<u32>], fully: bool) -> bool {
    assert!(ranges.len() == 2);
    let first = ranges.get(0).unwrap();
    let second = ranges.get(1).unwrap();
    match fully {
        true => first.is_subset(second) || second.is_subset(first),
        false => !first.is_disjoint(second),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(part2(input), 4);
    }
}
