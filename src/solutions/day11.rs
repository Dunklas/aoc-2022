use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref MANY_NUMBERS: Regex = Regex::new(r"(\d+),??").unwrap();
    static ref TRAILING_NUMBER: Regex = Regex::new(r"(\d+)$").unwrap();
    static ref OPERATION: Regex = Regex::new(r"(\*|\+) (old|\d+)$").unwrap();
}

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let monkeys = parse(input);
    let mut counts = simulate(monkeys, 20, Box::new(|x| x / 3));
    counts.sort();
    counts.into_iter().rev().take(2).product()
}

fn part2(input: &str) -> u64 {
    let monkeys = parse(input);
    let common: u64 = monkeys.iter().map(|m| m.test_value).product();
    let mut counts = simulate(monkeys, 10000, Box::new(move |x| x % common));
    counts.sort();
    counts.into_iter().rev().take(2).product()
}

fn simulate(mut monkeys: Vec<Monkey>, n: usize, relief: Box<dyn Fn(u64) -> u64>) -> Vec<u64> {
    let mut counts = vec![0u64; monkeys.len()];
    for _ in 0..n {
        for c in 0..monkeys.len() {
            let current = monkeys.get_mut(c).unwrap();
            let mut throws = Vec::<(u64, usize)>::new();
            while let Some(item) = current.items.pop_front() {
                let worry_level = relief(current.inspect(item));
                counts[c] += 1;
                throws.push((worry_level, current.calculate_target(worry_level)));
            }
            throws.into_iter().for_each(|(item, target)| {
                monkeys.get_mut(target).unwrap().items.push_back(item);
            })
        }
    }
    counts
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|raw_monkey| {
            let lines = raw_monkey.lines().collect::<Vec<&str>>();
            let operation_raw = OPERATION.captures(lines[2]).unwrap();
            Monkey {
                items: MANY_NUMBERS
                    .captures_iter(lines[1])
                    .map(|cap| cap[1].parse().unwrap())
                    .collect(),
                operation: (operation_raw[1].to_owned(), operation_raw[2].to_owned()),
                test_value: TRAILING_NUMBER.captures(lines[3]).unwrap()[1]
                    .parse()
                    .unwrap(),
                true_target: TRAILING_NUMBER.captures(lines[4]).unwrap()[1]
                    .parse()
                    .unwrap(),
                false_target: TRAILING_NUMBER.captures(lines[5]).unwrap()[1]
                    .parse()
                    .unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: (String, String),
    test_value: u64,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn inspect(&mut self, worry: u64) -> u64 {
        let operation_operand = match self.operation.1.as_str() {
            "old" => worry,
            _ => self.operation.1.parse().unwrap(),
        };
        match self.operation.0.as_str() {
            "*" => worry * operation_operand,
            "+" => worry + operation_operand,
            _ => panic!("Unexpected operator: {}", self.operation.0),
        }
    }
    fn calculate_target(&mut self, worry: u64) -> usize {
        let test = worry % self.test_value == 0;
        match test {
            true => self.true_target,
            false => self.false_target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }

    #[test]
    fn test_part1() {
        assert_eq!(10605, part1(input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, part2(input()));
    }
}
