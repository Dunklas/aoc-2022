use std::cmp::Ordering;

pub fn run(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    parse(input).chunks(2).enumerate()
        .filter(|(_, packets)| packets[0] < packets[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = parse(input);
    packets.append(&mut vec![Value::List("[[2]]".to_owned()), Value::List("[[6]]".to_owned())]);
    packets.sort();
    packets.into_iter().enumerate()
        .filter(|(_, p)| p.is_separator())
        .map(|(i, _)| i + 1)
        .product()
}

fn parse(input: &str) -> Vec<Value> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| Value::List(line.to_owned()))
        .collect()
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::Integer(l) => match other {
                Value::Integer(r) => l.cmp(r),
                Value::List(_) => Value::List(format!("[{}]", l)).cmp(other)
            },
            Value::List(_) => match other {
                Value::Integer(r) => self.cmp(&Value::List(format!("[{}]", r))),
                Value::List(_) => {
                    let mut l_iter = self.iter();
                    let mut r_iter = other.iter();
                    while let Some(l) = l_iter.next() {
                        let result = match r_iter.next() {
                            Some(r) => l.cmp(&r),
                            None => return Ordering::Greater,
                        };
                        if let Ordering::Equal = result {
                            continue;
                        }
                        return result;
                    };
                    if let Some(_) = r_iter.next() {
                        return Ordering::Less
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Value {
    Integer(u32),
    List(String)
}

impl Value {
    fn iter<'a>(&'a self) -> ValueIterator<'a> {
        match self {
            Value::List(s) => ValueIterator { content: &s[1..s.len() - 1], pos: 0 },
            _ => panic!("Not a list")
        }
    }
    fn is_separator(&self) -> bool {
        match self {
            Value::List(c) => c == "[[2]]" || c == "[[6]]",
            _ => false
        }
    }
}

struct ValueIterator<'a> {
    content: &'a str,
    pos: usize
}

impl<'a> Iterator for ValueIterator<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.content.chars().enumerate().skip(self.pos);
        while let Some((i, c)) = iter.next() {
            let (len, v) = match c {
                '[' => match find_matching(self.content, self.pos) {
                    Some(end) => (end + 1 - self.pos, Some(Value::List(String::from(&self.content[i..end+1])))),
                    None => panic!("No matching end bracket found")
                },
                ',' => (1, None),
                c if c.is_numeric() => {
                    let number: u32 = match self.content[self.pos..self.content.len()].find(",") {
                        Some(i) => self.content[self.pos..self.pos+i].parse().unwrap(),
                        None => self.content[self.pos..self.content.len()].parse().unwrap()
                    };
                    (number.to_string().len(), Some(Value::Integer(number)))
                },
                _ => panic!("Unexpected char")
            };
            self.pos += len;
            if let Some(v) = v {
                return Some(v);
            }
        }
        None
    }
}

fn find_matching(input: &str, start: usize) -> Option<usize> {
    let mut count = 0;
    for (i, c) in input.chars().enumerate().skip(start) {
        match c {
            '[' => count += 1,
            ']' => {
                count -= 1;
                if count == 0 {
                    return Some(i);
                }
            },
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        return "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 140);
    }
}