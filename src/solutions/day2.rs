pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(" ").map(|mv| normalize(mv)).collect::<Vec<_>>())
        .map(|moves| round_points(moves[1], moves[0]) + moves[1] + 1)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(" ").map(|mv| normalize(mv)).collect::<Vec<_>>())
        .map(|line| {
            match line[1] {
                0 => 0 + (line[0] + 2) % 3 + 1, // lose
                1 => 3 + line[0] + 1,           // draw
                2 => 6 + (line[0] + 1) % 3 + 1, // win
                _ => panic!("Invalid state"),
            }
        })
        .sum()
}

fn normalize(input: &str) -> u32 {
    match input {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("Invalid state"),
    }
}
fn round_points(slf: u32, other: u32) -> u32 {
    match (other + 3 - slf) % 3 {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => panic!("Invalid state"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "A Y
B X
C Z";
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn part2_test() {
        let input = "A Y
B X
C Z";
        assert_eq!(part2(input), 12);
    }
}
