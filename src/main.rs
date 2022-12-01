use std::fs;
fn main() {
    let raw_input = fs::read_to_string("data/day1.txt")
        .expect("Unable to parse file");

    let stacks = split_stacks(&raw_input);
    let elven_amount = calc_max_sum(stacks);
    println!("Found the max amount of snacks: {}", elven_amount.iter().max().unwrap());
}

fn calc_max_sum(elves: Vec<Vec<u32>>) -> Vec<u32> {
    elves.iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
}

fn split_stacks(input: &str) -> Vec<Vec<u32>> {
    let elements = input.split("\n\n");
    elements.map(|l| {
        l.split('\n')
            .flat_map(|v| v.parse::<u32>())
            .collect::<Vec<u32>>()
    }).collect()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn day1() {
        let input = data();
        let parsed = split_stacks(input);
        //println!("{parsed:?}");
        //assert_eq!(6, parsed.len());  // Should be 6 but I'm ignoring empty stacks for the first part
        assert_eq!(5, parsed.len());

        let maxes = calc_max_sum(parsed);
        println!("{maxes:?}");
        assert_eq!(&24000, maxes.iter().max().unwrap());
    }

    fn data() -> &'static str {
        r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#
    }
}
