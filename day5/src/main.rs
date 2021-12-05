use std::{error::Error, num::ParseIntError, str::FromStr, collections::{HashMap, hash_map::Entry}};
use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("Can't find input");

    let all_vents :Vec<Vent> = file_content.lines()
            .map(|x| x.parse::<Vent>().unwrap())
             .collect();


    println!("Found {} overlapping vents" , count_double_points( &all_vents, false));
    println!("Found {} overlapping vents with diagonal " , count_double_points( &all_vents, true));
}

#[derive(Debug)]
struct Vent {
    start: (i32, i32),
    end: (i32, i32),
}


fn count_double_points( vents : &Vec<Vent>, diagonal: bool ) -> i32 {

    let mut matched_points: HashMap<(i32,i32), i32> = HashMap::new();

    let all_points = vents.iter()
        .flat_map( | x | x.get_points(diagonal))
        .fold(matched_points,  |mut accum,dot |{

            let entry = accum.entry(dot);

            match entry {
                Entry::Vacant(to_insert) => {
                    accum.insert(dot, 1);
                }
                Entry::Occupied(mut prev) => {
                    let x = prev.get_mut();
                    *x+=1;
                }
            }

            accum
        });

    all_points.into_iter()
        .map(|x| x.1)
        .filter( |value| value >= &2)
        .count() as i32
}

impl Vent {

    pub fn max_x( &self) -> i32 {
        match self.start.0.cmp(&self.end.0)  {
            std::cmp::Ordering::Less => {
                self.end.0
            },
            _ => {
                self.start.0
            }
        }
    }

    pub fn max_y( &self) -> i32 {
        match self.start.1.cmp(&self.end.1)  {
            std::cmp::Ordering::Less => {
                self.end.1
            },
            _ => {
                self.start.1
            }
        }
    }

    pub fn get_points(&self, diagonal: bool) -> Vec<(i32,i32)> {

        let  mut c : Vec<(i32,i32)> = vec!(  );

        let range_vert = (self.start.1 - self.end.1).abs()+1;
        let range_horiz = (self.start.0 - self.end.0).abs()+1;
        let vert_dir = (self.start.1 -self.end.1).signum() * -1;
        let horiz_dir = (self.start.0 - self.end.0).signum() * -1;

        if self.start.0 == self.end.0 {

            for y in 0..range_vert {
                c.push( (self.start.0 , self.start.1+(y*vert_dir) ) );
            }
        } else if self.start.1 == self.end.1 {

            for x in 0..range_horiz {
                c.push( (self.start.0+(x*horiz_dir) , self.start.1 ) );
            }

        } else if diagonal {

            // diagonal line of 45 degrees

            let mut tot : Vec<(i32,i32)>= (0..range_horiz ).zip( 0..range_vert)
                    .map( |p| {
                        (self.start.0+( p.0*horiz_dir), self.start.1+( p.1*vert_dir) )
                    })
                    .collect();

            c.append(&mut tot);
 
        } else {
            // skip
        }

        //println!("found points {:?} ", c);
        c
    }

}

impl FromStr for Vent {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split(" -> ")
            .flat_map(|couple| couple.split(','))
            .map(|num| num.parse().unwrap() )
            .collect::<Vec<i32>>();

        Ok(Vent {
            start: (numbers[0], numbers[1]),
            end: (numbers[2], numbers[3]),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::{Vent, count_double_points};

    fn get_test_data() -> Vec<Vent> {
        r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#
            .lines()
            .map(|x| x.parse::<Vent>().unwrap())
            .collect()
    }
    #[test]
    fn test_parse() {

        let list = get_test_data();

        assert_eq!(10, list.len());

        assert_eq!( 0, list[0].start.0);
        assert_eq!( 2, list[9].end.1);

        assert_eq!(5,count_double_points( &list, false));
        assert_eq!(12,count_double_points( &list, true));

    }
}
