
pub fn execute( raw_input: &str) {

    let counts: Vec<u32>= raw_input.lines().flat_map(|p| p.parse::<u32>()).collect();

    if  let Some(result) = find_sum_equal( &counts, 2020) {

        println!("Result {:?} and product = {} ", &result, ( result.0*result.1));
    } else {
        panic!("Result not found?!?");
    }


    if let Some(result) = find_sum_of_three(&counts, 2020) {
        println!("Result {:?} and product = {} ", &result, ( result.0*result.1*result.2));
    }else {
        panic!("Result not found?!?");
    }

}

pub fn find_sum_of_three( list: &[u32], sum: u32) -> Option<(u32,u32,u32)> {
    for i in 0..list.len() {
        for x in 1..list.len() {
            for z in 2..list.len() {
                if ( list[x] + list[i] + list[z]) == sum {
                    return Some( ( list[x], list[i] , list[z]) );
                }
            }
        }
    }
    None
}

pub fn find_sum_equal( list: &[u32], sum: u32) -> Option<(u32,u32)> {

    for i in 0..list.len() {
        for x in 1..list.len() {
            if ( list[x] + list[i] ) == sum {
                return Some( ( list[x], list[i]) );
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_sum(){
        
        let list = prepare();
        //assert_eq!(Some((299, 1721)) , find_sum_equal(&list, 2020));
        if let Some(result) = find_sum_equal(&list, 2020) {
            assert_eq!(514579 , result.0 * result.1);
        } else {
            assert_eq!(1,0);
        }

    }

    #[test]
    fn test_three_sum(){

        let list = prepare();
        if let Some(result) = find_sum_of_three(&list, 2020) {
            assert_eq!(241861950 , result.0 * result.1 * result.2);
        } else {
            assert_eq!(1,0);
        }

    }

    fn prepare() -> Vec<u32> {
        data().lines().flat_map(|p| p.parse::<u32>()).collect()
    }


    fn data( ) -> &'static str {
        r#"1721
979
366
299
675
1456"#
    }
}