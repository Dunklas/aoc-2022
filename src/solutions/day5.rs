use regex::Regex;

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let instructions = parse_instructions(input);
    instructions.iter().for_each(|(num, source, target)| {
        (0..*num).for_each(|_| {
            let tmp = stacks[*source].pop().unwrap();
            stacks[*target].push(tmp);
        });
    });
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let instructions = parse_instructions(input);
    instructions.iter().for_each(|(num, source, target)| {
        let split_i = stacks[*source].len() - *num as usize;
        let mut to_move = stacks[*source].split_off(split_i);
        stacks[*target].append(&mut to_move);
    });
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let stacks_raw = input.split("\n\n").collect::<Vec<_>>()[0];
    let max = stacks_raw.lines().map(|l| l.len()).max().unwrap();
    (1..max)
        .step_by(4)
        .map(|column_i| {
            stacks_raw
                .lines()
                .rev()
                .filter_map(|line| line.chars().nth(column_i))
                .filter(|c| c.is_alphabetic())
                .collect()
        })
        .collect()
}

fn parse_instructions(input: &str) -> Vec<(u32, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input.split("\n\n").collect::<Vec<_>>()[1]
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let num = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<u32>()
                .unwrap();
            let source = caps
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap()
                - 1;
            let target = caps
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap()
                - 1;
            (num, source, target)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part1(input), "CMZ".to_owned());
    }

    #[test]
    fn test_part2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part2(input), "MCD".to_owned());
    }
}
