use std::{
    env,
    io::{self, Read},
};

use solutions::{
    day1, day10, day11, day12, day13, day14, day15, day2, day3, day4, day5, day6, day7, day8, day9,
};

mod solutions;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    match day.as_str() {
        "1" => {
            day1::run(&input);
        }
        "2" => {
            day2::run(&input);
        }
        "3" => {
            day3::run(&input);
        }
        "4" => {
            day4::run(&input);
        }
        "5" => {
            day5::run(&input);
        }
        "6" => {
            day6::run(&input);
        }
        "7" => {
            day7::run(&input);
        }
        "8" => {
            day8::run(&input);
        }
        "9" => {
            day9::run(&input);
        }
        "10" => {
            day10::run(&input);
        }
        "11" => {
            day11::run(&input);
        }
        "12" => {
            day12::run(&input);
        }
        "13" => {
            day13::run(&input);
        }
        "14" => {
            day14::run(&input);
        }
        "15" => {
            day15::run(&input);
        }
        _ => panic!("Solution not found"),
    };
}
