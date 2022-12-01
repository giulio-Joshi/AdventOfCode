use std::fs;
fn main() {
    let raw_input = fs::read_to_string("data/day1.txt")
        .expect("Unable to parse file");

    let stacks = split_stacks(&raw_input);
    let mut elven_amount = calc_max_sum(stacks);
    println!("Found the max amount of snacks: {}", get_top_snack_holder(&mut elven_amount,1));
    println!("Found the three max amount of snacks: {}", get_top_snack_holder(&mut elven_amount,3));
}

fn get_top_snack_holder( elven_list: &mut [u32] , number: usize) -> u32 {

    elven_list.sort_by(|a,b | b.cmp(a));
    let actual_max = if number > elven_list.len() {
        elven_list.len()
    } else { 
        number
    };
    elven_list[0..actual_max].iter().sum()
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
        // Should be 6 but I'm ignoring empty stacks for the first part (proved to be unuseful)
        //assert_eq!(6, parsed.len()); 
        assert_eq!(5, parsed.len());

        let mut maxes = calc_max_sum(parsed);
        assert_eq!(24000, get_top_snack_holder(&mut maxes,1 ));
        assert_eq!(45000, get_top_snack_holder(&mut maxes,3 ));
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
