use std::fs;
fn main() {
    
    let file_content = fs::read_to_string("day9/input.txt").expect("Can't find input");
    let result = find_low( &convert(&file_content)) ;
    let total : u32 = result.into_iter().map( |z|  z as u32 +1_u32).sum() ;
    println!("Solution: {}", total);
}


fn find_low( points: &Vec<Vec<char>>) -> Vec<u8> {

    let mut all_lows: Vec<char> = filter_slice(&points[0..2], Some(true));
    

    all_lows.append(&mut points.windows(3)
            .flat_map( |m| filter_slice(m, None))
            .collect());

    all_lows.append(&mut filter_slice( &points[points.len()-2..points.len()], Some(false)));

    all_lows.into_iter()
        .map(|x| x as u8 - b'0' )
        .collect()
}


fn filter_slice( x: &[Vec<char>], head : Option<bool>) -> Vec<char> {

    let mut all_results= vec!();
    let mut relative_row = 1;

    if let Some(is_head) = head {
        if is_head {
            relative_row = 0;
        } else {
            relative_row = 1;
        }
    }

    for i in 0..x[0].len() {

        if is_less_in_group( x[relative_row][i], i, x , head) {
            all_results.push(  x[relative_row][i] );
        }
    }
    all_results
}

fn convert( input: &str ) -> Vec<Vec<char>> {

    input.lines()
        .map(|z| z.chars().collect::<Vec<char>>() )
        .collect()
}

fn is_less_in_group( value: char, index: usize, group: &[Vec<char>], head: Option<bool>) -> bool {

    let mut is_less_in_group = true;
    let mut relative_row = 1;


    // head/tail/central part
    match head {
        Some(true) => {
            relative_row= 0;
            is_less_in_group &=  value < group[1][index];
        },
        Some(false) => {
            relative_row= 1;
            is_less_in_group &=  value < group[0][index];
        },
        None => {
            is_less_in_group &=  value < group[0][index] && value < group[2][index];
        }
    }

    // left boundary
    if index > 0 {
        is_less_in_group &= value < group[relative_row][index-1];
    }


    // right boundary
    if index < group[0].len()-1 {
        is_less_in_group &= value < group[relative_row][index+1];
    }

    is_less_in_group
}


#[cfg(test)]
mod test {
    use crate::{find_low, convert};


    #[test]
    fn test_0 () {

        let expected = vec!( 1,0,5,5);

        let found = find_low( &convert(data())) ;


        assert_eq!(expected, found);
        let total : u32 = found.into_iter().map( |z|  z as u32 +1_u32).sum() ;
        assert_eq!(15,total);
    }

    fn data() -> &'static str {
        r#"2199943210
3987894921
9856789892
8767896789
9899965678"#
    }
}