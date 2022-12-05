use std::{
    env,
    io::{self, Read},
};

use solutions::{day1, day2, day3, day4, day5};

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    match day.as_str() {
        "1" => {
            day1::run(&input);
        },
        "2" => {
            day2::run(&input);
        },
        "3" => {
            day3::run(&input);
        },
        "4" => {
            day4::run(&input);
        },
        "5" => {
            day5::run(&input);
        }
        _ => panic!("Solution not found"),
    };
}
