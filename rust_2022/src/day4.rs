
pub fn execute( input: &str) {

    let turns = turn_list(input);
    let overlapping_turns = calc_overlapping(&turns);
    println!("Found {overlapping_turns} completely overlapping");
    let partial_overlap_turns = calc_partial_overlap(&turns);
    println!("Found {partial_overlap_turns} partially overlapping");
}

fn calc_partial_overlap( turns: &[Vec<u8>]) -> usize {

    turns.chunks(2)
        .filter(|&how| {

            let range1 = how[0][0]..=how[0][1];
            let range2  = how[1][0]..=how[1][1];

            for i in how[1][0]..=how[1][1] {
                if range1.contains(&i) {
                    return true;
                }
            }
            for i in how[0][0]..=how[0][1] {
                if range2.contains(&i) {
                    return true;
                }
            }
            false
            
            
    }).count()
}

fn calc_overlapping( turns: &[Vec<u8>]) -> usize {

    turns.chunks(2)
        .filter(|&how| {
            (how[0][0] <= how[1][0] && how[0][1] >= how[1][1]) 
            || (how[1][0] <= how[0][0] && how[1][1] >= how[0][1]) 
    }).count()
}

fn turn_list( input: &str) -> Vec<Vec<u8>> {

    let mut turns:Vec<Vec<u8>>  = vec![];

    input.lines()
        .for_each(|l| {

            let elves =l.split(',')
                .collect::<Vec<&str>>();

           let couple = elves.iter().map( |&x| {
                x.split('-').map( |v| v.parse::<u8>().unwrap()).collect::<Vec<u8>>()
           }).collect::<Vec<Vec<u8>>>();

           turns.append(&mut couple.clone());
        });

    turns
}


#[cfg(test)]
mod test {
    use crate::day4::{calc_overlapping, calc_partial_overlap};

    use super::turn_list;


    #[test]
    fn test_turn_list(){

        let turns = turn_list(data());

        println!("{turns:?}");
        assert_eq!(12, turns.len());

        assert_eq!(2, calc_overlapping(&turns));
        assert_eq!(4, calc_partial_overlap(&turns));
    }

    fn data() -> &'static str {
        r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
    }
}