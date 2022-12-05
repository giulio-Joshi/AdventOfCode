use std::ops::AddAssign;


///
/// Main function for day3
/// 
pub fn execute( input: &str) {
    let all_sacks  = input.lines().map(sacks).collect::<Vec<(String, String)>>();
    let as_singles = input.lines().map(|x|x.to_owned())
        .collect::<Vec<String>>();

    println!("Found sum: {} ", sum_common_content(&all_sacks));
    println!("Found common sums: {} ",sum_common_by_group( &as_singles, 3));
}

fn sum_common_content( sacks: &[(String,String)]) -> u32 {

    let all_things= sacks.iter()
        .map(find_common)
        .collect::<Vec<String>>()
        .iter()
        .map(letter_convert)
        .collect::<Vec<u32>>();

    all_things.iter().sum()
}

fn sum_common_by_group( sacks: &[String], group_size: usize ) -> u32 {

    sacks.chunks(group_size)
        .map(find_common_chunks)
        .collect::<Vec<String>>()
        .iter()
        .map(letter_convert)
        .collect::<Vec<u32>>()
        .iter()
        .sum()

}

fn find_common_chunks( chunk: &[String]) -> String {
    
    let tmp_result = chunk.windows(2)
        .map( | w | { 
            find_common( &( w[0].clone(), w[1].clone()) )
        })
        .collect::<Vec<String>>();
    
    if tmp_result.len() > 1 {
        find_common_chunks(&tmp_result)
    } else {

        tmp_result[0].to_owned()
    }
}

fn find_common(sack: &(String,String)) -> String {

    let mut content0 = sack.0.clone();
    content0.retain(|f| {

        sack.1.contains(f)
    });

    let mut content1 = sack.1.clone();
    content1.retain(|f| sack.0.contains(f));

    let result = &mut String::new();
    for letter in content0.chars() {
        let value_exists = result.contains(letter);
        if !value_exists {
            result.add_assign(&letter.to_string());
        }
    }

    result.to_string()
}

fn sacks( data : &str) -> (String, String) {
    let half = data.len() / 2;
    ( data[0..half].to_owned() , data[half..].to_owned() )
}

fn letter_convert( data: &String) -> u32 {
    let one_letter = data.bytes().next().unwrap();
    if one_letter > b'Z' {
        (one_letter - (b'a' -1)  ) as u32
    } else {
        (26 + one_letter - (b'A' -1) ) as u32
    }
}

#[cfg(test)]
mod test {
    use crate::day3::*;

    #[test]
    pub fn sacks_split( ){
        let all_sacks = split_data();
        assert_eq!(6, all_sacks.len());

        all_sacks.iter().take(1)
            .for_each(|x| {
                assert_eq!("vJrwpWtwJgWr", x.0);
                assert_eq!("hcsFMMfFFhFp",x.1);

                assert_eq!("p", find_common(x));
            });

            assert_eq!(157, sum_common_content( &all_sacks));
    }

    #[test]
    pub fn sacks_split_three( ) {
        let all_sacks = data().split('\n')
            .map(|x|x.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(70, sum_common_by_group(&all_sacks, 3));
    }

    #[test]
    fn test_ascii_conv( )  {
        assert_eq!(1, letter_convert( &"a".to_owned() ));
        assert_eq!(26, letter_convert( &"z".to_owned() ));
        assert_eq!(27, letter_convert( &"A".to_owned() ));
        assert_eq!(52, letter_convert( &"Z".to_owned()));
    }

    fn split_data() -> Vec<(String,String)> {

        data().lines().map(sacks).collect::<Vec<(String, String)>>()
    }


    fn data() -> &'static str {
        r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
    }
}