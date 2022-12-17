use std::{collections::HashMap, str::FromStr};

use super::coordinate::Coordinate;

pub struct Grid<S>
where
    S: FromStr + PartialEq,
{
    data: HashMap<Coordinate, S>,
}

impl<S> Grid<S>
where
    S: FromStr + PartialEq,
{
    pub fn find(&self, v: S) -> Vec<&Coordinate> {
        self.data
            .iter()
            .filter(|(_, value)| **value == v)
            .map(|(coordinate, _)| coordinate)
            .collect()
    }
    pub fn find_first(&self, v: S) -> Option<&Coordinate> {
        self.find(v).into_iter().nth(0)
    }
    pub fn value_at(&self, c: &Coordinate) -> Option<&S> {
        self.data.get(c)
    }
    pub fn adjacent(&self, source: &Coordinate) -> Vec<&Coordinate> {
        vec![
            self.data
                .get_key_value(&Coordinate::new(source.x, source.y + 1)),
            self.data
                .get_key_value(&Coordinate::new(source.x + 1, source.y)),
            self.data
                .get_key_value(&Coordinate::new(source.x, source.y - 1)),
            self.data
                .get_key_value(&Coordinate::new(source.x - 1, source.y)),
        ]
        .into_iter()
        .filter_map(|x| x)
        .map(|(coordinate, _)| coordinate)
        .collect()
    }
}

impl<S> Grid<S>
where
    S: FromStr + PartialEq,
{
    pub fn parse(input: &str) -> Grid<S> {
        let mut data: HashMap<Coordinate, S> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data.insert(
                    Coordinate::new(x as i32, y as i32),
                    match c.to_string().parse::<S>() {
                        Ok(v) => v,
                        Err(_) => panic!("Failed to parse grid"),
                    },
                );
            }
        }
        Grid { data }
    }
}
