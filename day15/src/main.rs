fn main() {
    println!("Hello, world!");
}


fn calc_path( ceiling: &str) -> u32 {

    /*let mut visited : Vec<usize> = vec!();


    let rows: Vec<&str> = ceiling.lines().collect();

    let mut width= ceiling.lines().next().unwrap().trim().len();

    visited.push(0);
    let mut position = 0;
    let mut sum : u32= ceiling[0..1].chars().take(1).map(|z| (z as u8 - b'0') as u32).sum();

    while position != (width*2) {


        let target_y = position / width;
        let target_x  = 

    }*/


    0
}


#[cfg(test)]
mod test {
    use crate::calc_path;



    #[test]

    fn part_01 () {


        assert_eq!( 40, calc_path( data() ));

    }

    fn data() -> &'static str {
        r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#
    }
}