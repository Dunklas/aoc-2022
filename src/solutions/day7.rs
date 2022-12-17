use std::collections::BTreeMap;

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input).unwrap());
}

fn part1(input: &str) -> usize {
    let files = parse(input);
    files
        .iter()
        .filter(|(_, file)| **file == File::Directory)
        .map(|(path, _)| size_of(&path, &files))
        .filter(|size| *size <= 100000)
        .sum()
}

fn part2(input: &str) -> Option<usize> {
    let files = parse(input);
    let needed = 30000000 - (70000000 - size_of("/", &files));
    files
        .iter()
        .filter(|(_, file)| **file == File::Directory)
        .map(|(path, _)| size_of(path, &files))
        .filter(|size| *size >= needed)
        .min()
}

fn size_of(dir: &str, files: &BTreeMap<String, File>) -> usize {
    files
        .iter()
        .filter(|(path, _)| path.starts_with(dir))
        .filter_map(|(_, file)| match *file {
            File::Data(s) => Some(s),
            File::Directory => None,
        })
        .sum()
}

fn parse(input: &str) -> BTreeMap<String, File> {
    let mut current: Vec<&str> = Vec::new();
    let mut files: BTreeMap<String, File> = BTreeMap::new();
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match line {
            "$ cd .." => {
                current.pop();
            }
            line if line.starts_with("$ cd ") => {
                current.push(parts[2]);
            }
            line if !line.starts_with("$") => {
                files.insert(
                    absolute_path(&current, &parts[1]),
                    match line.starts_with("dir") {
                        true => File::Directory,
                        false => File::Data(parts[0].parse().unwrap()),
                    },
                );
            }
            _ => {}
        }
    }
    files
}

fn absolute_path(current: &Vec<&str>, file: &str) -> String {
    format!("/{}/{}", current.join("/"), file)
}

#[derive(PartialEq, Eq, Debug)]
enum File {
    Directory,
    Data(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
    }

    #[test]
    fn test_part1() {
        assert_eq!(95437, part1(input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24933642, part2(input()).unwrap());
    }
}
