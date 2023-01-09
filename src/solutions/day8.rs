pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input).unwrap());
}

fn part1(input: &str) -> usize {
    let trees = parse(input);
    trees
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| is_visible((y, *x), &trees))
                .count()
        })
        .sum()
}

fn part2(input: &str) -> Option<usize> {
    let trees = parse(input);
    trees
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| scenic_score((y, x), &trees))
                .collect::<Vec<usize>>()
        })
        .max()
}

fn is_visible(pos: (usize, usize), trees: &Vec<Vec<u32>>) -> bool {
    let current_height = trees[pos.0][pos.1];
    vec![
        (0..pos.0).map(|y| trees[y][pos.1]).collect(),
        (0..pos.1).map(|x| trees[pos.0][x]).collect(),
        (pos.0 + 1..trees.len()).map(|y| trees[y][pos.1]).collect(),
        (pos.1 + 1..trees[0].len())
            .map(|x| trees[pos.0][x])
            .collect(),
    ]
    .into_iter()
    .any(|dir: Vec<u32>| dir.into_iter().all(|h| h < current_height))
}

fn scenic_score(pos: (usize, usize), trees: &Vec<Vec<u32>>) -> usize {
    let current_height = trees[pos.0][pos.1];
    vec![
        (0..pos.0).map(|y| trees[y][pos.1]).rev().collect(),
        (0..pos.1).map(|x| trees[pos.0][x]).rev().collect(),
        (pos.0 + 1..trees.len()).map(|y| trees[y][pos.1]).collect(),
        (pos.1 + 1..trees[0].len())
            .map(|x| trees[pos.0][x])
            .collect(),
    ]
    .into_iter()
    .map(|dir: Vec<u32>| {
        let mut score = 0;
        for height in dir {
            score += 1;
            if height >= current_height {
                break;
            }
        }
        score
    })
    .product()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(21, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(Some(8), part2(input));
    }
}
