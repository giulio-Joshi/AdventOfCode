use std::{collections::HashMap, fs};

fn main() {
    let file_content = fs::read_to_string("day14/input.txt").expect("Can't find input");
    let template = read_template(&file_content);
    let pairs = read_couples(&file_content);

    let result_in_10 = calc_most_less_minus(apply_pairs(&template, &pairs, 10));
    println!("Counted value part 2 {}", result_in_10);
    let result_in_40 = calc_most_less_minus(apply_pairs(&template, &pairs, 40));
    println!("Counted value part 2 {}", result_in_40);
}

fn calc_most_less_minus(element_count: HashMap<String, u64>) -> u64 {
    println!("{:?}", element_count);

    let most_common =
        element_count
            .iter()
            .fold(0_u64, |accum, p| if p.1.gt(&accum) { *p.1 } else { accum });

    let less_common =
        element_count.iter().fold(
            most_common,
            |accum, p| {
                if p.1.lt(&accum) {
                    *p.1
                } else {
                    accum
                }
            },
        );

    most_common - less_common
}

fn apply_pairs(template: &str, rules: &[Pair], steps: u64) -> HashMap<String, u64> {
    let mut all_pairs: HashMap<String, usize> = HashMap::new();

    for i in 0..(template.len() - 1) {
        let couple = &template[i..(i + 2)];
        *all_pairs.entry(String::from(couple)).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut result = all_pairs.clone();

        for (pair, dimension) in all_pairs.drain() {
            if let Some(rule) = rules.iter().find(|x| x.matches(&pair)) {
                let cp0 = [&pair[0..1], &rule.add].concat();
                *result.entry(cp0).or_insert(0) += dimension;

                let cp1 = [&rule.add, &pair[1..2]].concat();
                *result.entry(cp1).or_insert(0) += dimension;
            }

            *result.entry(pair).or_insert(dimension) -= dimension;
        }

        all_pairs = result.clone();
    }

    let mut start = HashMap::new();
    let last_letter = String::from(&template[(template.len() - 1)..(template.len())]);
    start.insert(last_letter, 1);

    all_pairs.into_iter().fold(start, |mut accum, p| {
        *accum.entry(String::from(&p.0[0..1])).or_insert(0) += p.1 as u64;
        accum
    })
}
struct Pair {
    couple: String,
    add: String,
}

impl Pair {
    fn new(data: &str) -> Self {
        let mut v = data.split(" -> ");
        Pair {
            couple: String::from(v.next().unwrap()),
            add: String::from(v.next().unwrap()),
        }
    }

    fn matches(&self, polymer_part: &str) -> bool {
        self.couple.eq(polymer_part)
    }
}

fn read_template(i: &str) -> String {
    i.lines().take(1).map(String::from).collect::<Vec<String>>()[0].to_owned()
}

fn read_couples(input: &str) -> Vec<Pair> {
    input.lines().skip(2).map(Pair::new).collect()
}

#[cfg(test)]
mod test {
    use crate::{apply_pairs, calc_most_less_minus, read_couples, read_template};

    #[test]
    fn steps() {
        let template = read_template(data());
        let pairs = read_couples(data());

        assert_eq!(
            1588,
            calc_most_less_minus(apply_pairs(&template, &pairs, 10))
        );
    }

    #[test]
    fn steps_40() {
        let template = read_template(data());
        let pairs = read_couples(data());

        assert_eq!(
            2188189693529,
            calc_most_less_minus(apply_pairs(&template, &pairs, 40))
        );
    }

    #[test]
    fn test_parsing() {
        let template = read_template(data());
        assert_eq!("NNCB", template);

        let pairs = read_couples(data());
        assert_eq!(16, pairs.len());
    }

    fn data() -> &'static str {
        r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#
    }
}
