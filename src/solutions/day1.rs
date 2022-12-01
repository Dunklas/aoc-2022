pub fn run(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.split("\n").map(|x| x.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut elfs: Vec<usize> = input
        .split("\n\n")
        .map(|group| group.split("\n").map(|x| x.parse::<usize>().unwrap()).sum())
        .collect();
    elfs.sort();
    elfs.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn part2_test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(part2(input), 45000);
    }
}
