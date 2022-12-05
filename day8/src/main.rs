use std::collections::HashMap;
use std::fs;
use std::string::ParseError;

fn main() {
    let file_content = fs::read_to_string("day8/input.txt").expect("Can't find input");

    let count: i32 = convert(&file_content).iter().map(|d| d.count_easy()).sum();
    println!("Found {} `easy digits`  ", count);

    let count: i32 = convert(&file_content)
        .iter()
        .map(|d| d.count_not_easy())
        .sum();
    println!("Found {} NOT easy digits  ", count);
}

fn convert(original: &str) -> Vec<Signal> {
    original.lines().map(|x| Signal::new(x).unwrap()).collect()
}

fn chars_to_u8(i: &str) -> Vec<u8> {
    i.chars().map(|z| z as u8).collect()
}

#[derive(Debug)]
struct Signal {
    patterns: HashMap<u8, Vec<u8>>,
    output: Vec<Vec<u8>>,
}

impl<'a> Signal {
    pub fn new(s: &str) -> Result<Self, ParseError> {
        let mut s = s.split(" | ");

        let patterns: Vec<&str> = s.next().unwrap().split(' ').map(|z| z.trim()).collect();
        let output: Vec<Vec<u8>> = s.next().unwrap().split(' ').map(chars_to_u8).collect();

        Ok(Signal {
            patterns: find_patterns(patterns),
            output,
        })
    }

    pub fn count_easy(&self) -> i32 {
        let easy_digits: Vec<u8> = vec![7, 4, 3, 2];

        self.output
            .iter()
            .filter(|&o| easy_digits.contains(&(o.len() as u8)))
            .count() as i32
    }

    pub fn count_not_easy(&self) -> i32 {
        let numbers: Vec<i32> = self
            .output
            .iter()
            .map(|out| {
                self.patterns
                    .iter()
                    .filter(|z| vec_content_matches(z.1, out))
                    .map(|filtered| *filtered.0 as i32)
                    .next()
                    .unwrap()
            })
            .collect();

        (numbers[0] * 1000) + (numbers[1] * 100) + (numbers[2] * 10) + numbers[3]
    }
}

fn vec_content_matches(v1: &[u8], v2: &[u8]) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    let cnd = v1.iter().filter(|z| !v2.contains(z)).count() == 0;

    let cnd2 = v2.iter().filter(|z| !v1.contains(z)).count() == 0;

    cnd && cnd2
}

fn find_patterns(list: Vec<&str>) -> HashMap<u8, Vec<u8>> {
    let mut result: HashMap<u8, Vec<u8>> = list
        .iter()
        .map(|&d| match d.len() {
            7 => Some((8, chars_to_u8(d))),
            3 => Some((7, chars_to_u8(d))),
            4 => Some((4, chars_to_u8(d))),
            2 => Some((1, chars_to_u8(d))),
            _ => None,
        })
        .flatten()
        .collect();

    list.iter()
        .map(|&d| {
            if d.len() == 5 {
                if let Some(conv) = is_5(d, &result) {
                    return Some((5_u8, conv));
                }
                if let Some(conv) = is_3(d, &result) {
                    return Some((3_u8, conv));
                }
                return Some((2, chars_to_u8(d)));
            } else if d.len() == 6 {
                if let Some(conv) = is_9(d, &result) {
                    return Some((9_u8, conv));
                }
                if let Some(conv) = is_6(d, &result) {
                    return Some((6_u8, conv));
                }
                return Some((0, chars_to_u8(d)));
            }
            None
        })
        .flatten()
        .collect::<Vec<(u8, Vec<u8>)>>()
        .into_iter()
        .for_each(|z| {
            result.insert(z.0, z.1);
        });

    result
}

fn is_6(test: &str, known: &HashMap<u8, Vec<u8>>) -> Option<Vec<u8>> {
    let mut c_vec = chars_to_u8(test);
    c_vec.retain(|v| !known.get(&1).unwrap().contains(v));

    if c_vec.len() == 5 {
        Some(chars_to_u8(test))
    } else {
        None
    }
}

fn is_9(test: &str, known: &HashMap<u8, Vec<u8>>) -> Option<Vec<u8>> {
    let mut c_vec = chars_to_u8(test);
    c_vec.retain(|v| !known.get(&4).unwrap().contains(v));
    c_vec.retain(|v| !known.get(&7).unwrap().contains(v));

    if c_vec.len() == 1 {
        Some(chars_to_u8(test))
    } else {
        None
    }
}

fn is_3(test: &str, known: &HashMap<u8, Vec<u8>>) -> Option<Vec<u8>> {
    let mut c_vec = chars_to_u8(test);
    c_vec.retain(|v| !known.get(&4).unwrap().contains(v));
    if c_vec.len() == 2 {
        Some(chars_to_u8(test))
    } else {
        None
    }
}

fn is_5(test: &str, known: &HashMap<u8, Vec<u8>>) -> Option<Vec<u8>> {
    let mut c_vec = chars_to_u8(test);
    c_vec.retain(|v| !known.get(&7).unwrap().contains(v));

    let diff_from_4 = known
        .get(&4)
        .unwrap()
        .iter()
        .filter(|z| c_vec.contains(z))
        .count();

    if c_vec.len() == 3 && diff_from_4 == 2 {
        Some(chars_to_u8(test))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{convert, Signal};

    #[test]
    fn find_easy_digits() {
        let signals: Vec<Signal> = convert(data());
        println!("{:?}", signals[0].patterns);
        assert_eq!(26, signals.iter().map(|z| z.count_easy()).sum());
        assert_eq!(61229, signals.iter().map(|z| z.count_not_easy()).sum());
    }

    #[test]
    fn full_convert() {
        let x = convert(example_0());
        println!("{:?}", &x);
    }

    fn example_0() -> &'static str {
        r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#
    }

    fn data() -> &'static str {
        r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#
    }
}
