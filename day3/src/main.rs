use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("Can't find input");

    let file_content = file_content.lines().collect::<Vec<&str>>();

    let gamma = find_measure(&file_content, Measure::Gamma);
    let epsilon = find_measure(&file_content, Measure::Epsilon);

    println!("Power comsumption is {}", gamma * epsilon);
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

fn find_measure(vec: &[&str], m: Measure) -> u32 {
    let mut bits = String::from("");
    let count_half_entries = vec.len() / 2;
    let word_length = vec[0].len();

    for i in 0..word_length {
        let is_over_mid = vec
            .iter()
            .map(|&x| x.chars().nth(i).unwrap())
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

#[cfg(test)]
mod test {
    use crate::find_measure;

    #[test]
    fn simple_test_case() {
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

        let converted = original.lines().collect::<Vec<&str>>();

        assert_eq!(22, find_measure(&converted, crate::Measure::Gamma));
        assert_eq!(9, find_measure(&converted, crate::Measure::Epsilon));
    }
}
