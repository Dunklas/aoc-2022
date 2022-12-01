use std::{
    env,
    io::{self, Read},
};

use solutions::day1;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    match day.as_str() {
        "1" => {
            day1::run(&input);
        }
        _ => panic!("Solution not found"),
    };
}
