use std::collections::HashSet;

pub fn run(input: &str) {
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(first, second)| items_in_all(&[first, second]))
        .map(priority)
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(items_in_all)
        .map(priority)
        .sum()
}

fn items_in_all(compartments: &[&str]) -> HashSet<char> {
    assert!(!compartments.is_empty());
    compartments[0]
        .chars()
        .filter(|c| {
            compartments
                .iter()
                .all(|compartment| compartment.contains(*c))
        })
        .collect::<HashSet<_>>()
}

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 64 + 26,
        _ => {
            panic!("Unexpected char: {}", c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part2(input), 70);
    }
}
