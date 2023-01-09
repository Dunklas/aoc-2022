pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: \n{}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut signal_strengths: Vec<i64> = vec![];
    let mut x = 1;
    let mut cycle_count = 0;
    for (cycle_time, value) in parse(input) {
        for _ in 0..cycle_time {
            cycle_count += 1;
            if (cycle_count - 20) % 40 == 0 {
                signal_strengths.push(cycle_count * x);
            }
        }
        x += value;
    }
    signal_strengths.iter().sum()
}

fn part2(input: &str) -> String {
    let mut x = 1;
    let mut cycle_count = 0i64;
    let mut screen = [[' '; 40]; 6];
    for (cycle_time, value) in parse(input) {
        for _ in 0..cycle_time {
            cycle_count += 1;
            let col = (cycle_count - 1) % 40;
            let c = match col.abs_diff(x) <= 1 {
                true => '#',
                false => '.',
            };
            screen[((cycle_count - 1) / 40) as usize][col as usize] = c;
        }
        x += value;
    }
    render(screen)
}

fn render(screen: [[char; 40]; 6]) -> String {
    let mut result = String::new();
    for i in 0..screen.len() {
        for j in 0..screen[0].len() {
            result.push(screen[i][j]);
        }
        result.push('\n');
    }
    result
}

fn parse(input: &str) -> Vec<(u8, i64)> {
    input
        .lines()
        .map(|line| match line {
            "noop" => (1, 0),
            _ => (
                2,
                line.split(' ').collect::<Vec<&str>>()[1]
                    .parse::<i64>()
                    .unwrap(),
            ),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 13140);
    }

    #[test]
    fn test_part2() {
        println!("{}", part2(input()));
    }
}
