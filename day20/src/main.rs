use std::fs;

fn main() {
    let file_content = fs::read_to_string("day20/input.txt").expect("Can't find input");
    let conversion = parse_input(&file_content);

    println!("Found {} big image", conversion.2);

    let the_result = enhanche( &conversion.0, conversion.2 ,& conversion.1,2 );

    print_grid(conversion.2+5, &the_result);

}


fn enhanche( algo : &str, size:usize,  points: &[(i64,i64)] , passes: usize  ) -> Vec<(i64,i64)> {

    let mut the_points  : Vec<(i64,i64)> = vec!();


    for x  in -1..(size+2) as i64 {
        for  y in -1..(size+2) as i64 {

            let width= (1+passes%2) as i64;
            
            let  binary = [
                    to_binary(x-width, y-width, points ), to_binary(x, y-width, points), to_binary(x+width, y-width, points),
                    to_binary(x-width,   y  , points), to_binary(x,    y  , points), to_binary(x+1,    y  , points),
                    to_binary(x-width, y+width, points), to_binary(x, y+width, points), to_binary(x+width, y+width, points),
                ].concat();

            let element_pos = usize::from_str_radix(&binary, 2) .unwrap();
            if algo[element_pos..element_pos+1].eq("#") {
                the_points.push( (x,y));
            }
        }
    }

    print_grid(size+2, &the_points);

        
    if passes >1  {
        enhanche( algo, size+2, &the_points, passes-1)
     } else {
        the_points
     }

}

fn to_binary(x : i64,y:i64, points:&[(i64,i64)] ) -> &str{
    if points.contains( &(x,y) ) {
        return "1";
    } 
    "0"
}

fn parse_input(input : &str) -> (String, Vec<(i64,i64)>, usize) {

    let algo = input.lines().next().unwrap();
    let original : Vec<&str> = Iterator::collect(input.lines().skip(2));
    let mut points :Vec<(i64,i64)>= vec!();

    let dimension = original.len();

    original.into_iter()
        .enumerate()
        .for_each(|row| {

            (0..row.1.len()).for_each(|i| {
                if row.1[i..i+1].eq("#") {
                    points.push( ( row.0 as i64, i as i64));
                }

            });
        });

    (algo.to_string(), points, dimension) 
}


fn print_grid( size: usize, points: &[(i64,i64)] ) {

    for x  in -5..(size+3) as i64 {
        for  y in -5..(size+3) as i64 {
            print!("{}",&to_binary( x,y, points ));
        }
        println!();
    }

}

#[cfg(test)]
mod test {
    use crate::{parse_input, enhanche, print_grid};


    #[test]
    fn test_convert() {

        let conversion = parse_input(data());
        assert_eq!(10, conversion.1.len());
        assert_eq!(5, conversion.2);


        let enhanched = enhanche( &conversion.0, conversion.2 ,& conversion.1, 2 );
        print_grid(conversion.2+2, &enhanched );
        assert_eq!(35 , enhanched.len());

    }


    fn data() -> &'static str {
        r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#

    }
}