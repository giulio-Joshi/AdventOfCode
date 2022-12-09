use core::panic;
use std::{env, fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args
        .last()
        .unwrap_or(&"_".to_owned())
        .parse::<u8>()
        .unwrap_or(9);

    let input_file = format!("data/day{}.txt", day);
    let raw_input = fs::read_to_string(input_file).expect("Unable to parse file");

    match day {
        9 => day9::execute(&raw_input),
        8 => day8::execute(&raw_input),
        7 => day7::execute(&raw_input),
        6 => day6::execute(&raw_input),
        5 => day5::execute(&raw_input),
        4 => day4::execute(&raw_input),
        3 => day3::execute(&raw_input),
        2 => day2::execute(&raw_input),
        1 => day1::execute(&raw_input),
        _ => panic!("Day {day} not implemented"),
    }
}
