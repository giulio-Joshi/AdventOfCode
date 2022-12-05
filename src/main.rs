use std::env::args;
use std::fs;

mod day1; mod day2;

fn main() {

    let args : Vec<String> = args().collect();
    
    let day = args.last().unwrap_or(&"_".to_owned())
        .parse::<u8>()
        .unwrap_or(2);
    
    let input_file = format!("data/day{}.txt", day);    
    let raw_input = fs::read_to_string(input_file).expect("Unable to parse file");

    match day {
        //3 => day3::execute(&raw_input),
        2 => day2::execute(&raw_input),
        1 => day1::execute(&raw_input),
        _ => panic!("Day {day} not implemented"),

    }

}
