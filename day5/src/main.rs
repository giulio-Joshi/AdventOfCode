use std::{error::Error, num::ParseIntError, str::FromStr, collections::{HashMap, hash_map::Entry}};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Vent {
    start: (i32, i32),
    end: (i32, i32),
}


fn count_double_points( vents : &Vec<Vent> ) -> i32 {

    let mut matched_points: HashMap<(i32,i32), i32> = HashMap::new();

    let all_points = vents.iter()
        .flat_map( | x | x.get_points())
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

/***
 *  created because of diagonal lines
 * 
 * /
fn get_overlapping( board_dim: (i32,i32), min_value: usize, vents: Vec<Vent>) -> usize {

    //let mut matched_points: HashMap<(i32,i32), i32> = HashMap::new();

    let mut counter = 0;

    for x in 0..board_dim.0 {
        for y in 0..board_dim.1 {

            let tot = vents.iter()
                .filter( |v| v.touches((x,y)) )
                .count();

            if tot >= min_value   {
                counter+=tot;
            }

        }
    }

    counter

}
 */

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


    pub fn touches(&self, point: (i32,i32)) -> bool{

        //
        // Barbarically created using this solution
        //  https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment
        //

        let crossproduct= (point.1 - self.start.1 ) * ( self.end.0 -self.start.0) 
                            - (point.0 - self.start.0 ) * ( self.end.1 -self.start.1 );
    
        if crossproduct != 0 {
            return false;
        }

        let dotproduct   = (point.0 - self.start.0 ) * ( self.end.0 - self.start.0 ) 
            + ( point.1 - self.start.1) * (self.end.1 - self.start.1);

        if dotproduct <0 {
            return false;
        }

        let square_length_hba = (self.end.0 - self.start.0) * (self.end.0 - self.start.0) 
                        -  (self.end.1 - self.start.1) * (self.end.1 - self.start.1); 

        if dotproduct > square_length_hba {
            return false;
        }
        
        true
    }

    
    pub fn get_points(&self) -> Vec<(i32,i32)> {

        let  mut c : Vec<(i32,i32)> = vec!(  );

        if self.start.0 == self.end.0 {

            let range_vert = (self.start.1 - self.end.1).abs()+1;
            let vert_dir = (self.start.1 -self.end.1).signum() * -1;

            for y in 0..range_vert {
                c.push( (self.start.0 , self.start.1+(y*vert_dir) ) );
            }


        } 
        
        if self.start.1 == self.end.1 {

            let range_horiz = (self.start.0 - self.end.0).abs()+1;
            let horiz_dir = (self.start.0 - self.end.0).signum() * -1;
            for x in 0..range_horiz {
                c.push( (self.start.0+(x*horiz_dir) , self.start.1 ) );
            }

        }

        println!("found points {:?} ", c);
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

        assert_eq!(5,count_double_points( &list));

    }
}
