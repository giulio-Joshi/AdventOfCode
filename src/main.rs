use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    //let raw_input = fs::read_to_string("data/day1.txt").expect("Unable to parse file");
    //day1::execute(&raw_input);

    //let raw_input = fs::read_to_string("data/day2.txt").expect("Unable to parse file");
    //day2::execute(&raw_input);

    let raw_input = fs::read_to_string("data/day3.txt").expect("Unable to parse file");
    day3::execute(&raw_input);
}
