use std::cmp::Ordering;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("Can't find input");
    let file_content: Vec<String> = file_content.lines().map(String::from).collect();

    let gamma = find_measure(&file_content, Measure::Gamma);
    let epsilon = find_measure(&file_content, Measure::Epsilon);

    println!("Power comsumption is {}", gamma * epsilon);

    let co2_scrub = find_second_measure(&file_content, SubMeasure::CO2Scrub);
    let o2_gen = find_second_measure(&file_content, SubMeasure::O2Gen);

    println!("Life Support rating {}", co2_scrub * o2_gen);
}

enum Measure {
    Gamma,
    Epsilon,
}

impl Measure {
    pub fn over_threshold(x: &Measure) -> char {
        match x {
            Self::Gamma => '1',
            Self::Epsilon => '0',
        }
    }
    pub fn under_threshold(x: &Measure) -> char {
        match x {
            Self::Gamma => '0',
            Self::Epsilon => '1',
        }
    }
}

enum SubMeasure {
    CO2Scrub,
    O2Gen,
}

impl SubMeasure {
    pub fn determine_most_common(&self, v: &[(usize, char)]) -> char {
        let is_one = v.iter().filter(|&x| x.1 == '1').count();
        let is_zero = v.iter().filter(|&x| x.1 == '0').count();

        match self {
            SubMeasure::O2Gen => match is_one.cmp(&is_zero) {
                Ordering::Greater => '1',
                Ordering::Less => '0',
                Ordering::Equal => '1',
            },
            SubMeasure::CO2Scrub => match is_one.cmp(&is_zero) {
                Ordering::Greater => '0',
                Ordering::Less => '1',
                Ordering::Equal => '0',
            },
        }
    }
}

fn find_measure(vec: &[String], m: Measure) -> u32 {
    let mut bits = String::from("");
    let count_half_entries = vec.len() / 2;
    let word_length = vec[0].len();

    for i in 0..word_length {
        let is_over_mid = vec
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .filter(|&x| x == '1')
            .count();

        if is_over_mid >= count_half_entries {
            bits.push(Measure::over_threshold(&m));
        } else {
            bits.push(Measure::under_threshold(&m));
        }
    }

    u32::from_str_radix(bits.as_str(), 2).unwrap()
}

fn find_second_measure(vec: &[String], s: SubMeasure) -> u32 {
    let word_length = vec[0].len();
    let mut valid: Vec<usize> = (0..vec.len()).collect();

    for col in 0..word_length {
        let interesting: Vec<(usize, char)> = vec
            .iter()
            .enumerate()
            .map(|x| (x.0, x.1.chars().nth(col).unwrap()))
            .collect::<Vec<(usize, char)>>()
            .iter()
            .enumerate()
            .filter(|&x| valid.contains(&x.0))
            .map(|r| *r.1)
            .collect();

        let c = s.determine_most_common(&interesting);

        interesting.iter().filter(|&z| z.1.ne(&c)).for_each(|z| {
            if let Some(pos) = valid.iter().position(|x| *x == z.0) {
                valid.remove(pos);
            }
        });

        if valid.len() == 1 {
            return u32::from_str_radix(&vec[valid[0]], 2).unwrap();
        }
        if valid.is_empty() {
            valid = (0..word_length).collect();
        }
    }

    panic!("No available solution!??!");
}

#[cfg(test)]
mod test {
    use crate::{find_measure, find_second_measure, Measure, SubMeasure};

    fn data() -> Vec<String> {
        let original = String::from(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );

        original.lines().map(|x| x.into()).collect()
    }

    #[test]
    fn day3_01_test() {
        let converted = data();

        assert_eq!(22, find_measure(&converted, Measure::Gamma));
        assert_eq!(9, find_measure(&converted, Measure::Epsilon));
    }

    #[test]
    fn day3_02_test() {
        let original = data();

        assert_eq!(23, find_second_measure(&original, SubMeasure::O2Gen));
        assert_eq!(10, find_second_measure(&original, SubMeasure::CO2Scrub));
    }
}
