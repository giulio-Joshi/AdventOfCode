use std::{str::FromStr, io::{ErrorKind}, ops::Neg};

//
// I don't wanna create cuboids in memory as big as the numbers,
// I'll try to calculate intersections & numbers of lit cubes directly from data
//  ( please stay and enjoy failure... 💥!)

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug)]
struct  Cuboid {
    top: Coord,
    bottom: Coord,
    lit: bool
}

impl FromStr for Cuboid {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    
        let is_lit = s.split(' ').next().unwrap();
        let boundary = s.split(' ').nth(1).unwrap().split(',').collect::<Vec<&str>>();

        println!("{} - {:?}", &is_lit,&boundary );

        let mut cube  = Cuboid { top: Coord{ x:0,y:0,z:0} , bottom: Coord{ x:0,y:0,z:0} , lit: is_lit.eq("on")};

        boundary.into_iter()
            .for_each(|row| {

                let coordinate = row.split('=').next().unwrap();
                let start_stop : Vec<&str> = row.split('=').nth(1).unwrap().split("..").collect();
                match coordinate {
                    "x" => {
                        cube.top.x = start_stop[0].parse::<i64>().unwrap();
                        cube.bottom.x = start_stop[1].parse::<i64>().unwrap();
                    },
                    "y" => {
                        cube.top.y = start_stop[0].parse::<i64>().unwrap();
                        cube.bottom.y = start_stop[1].parse::<i64>().unwrap();
                    },
                    "z" => {
                        cube.top.z = start_stop[0].parse::<i64>().unwrap();
                        cube.bottom.z = start_stop[1].parse::<i64>().unwrap();
                    },
                    _ => {

                    }
                }

            });

        Ok(cube)
    }
}


impl Cuboid{
    fn count_cubes( &self ) -> i64 {
       self.width() * self.depth() * self.length()
    }

    fn length( &self ) -> i64 {
        1 + (self.top.x -self.bottom.x ).abs()
    }

    fn depth(&self) -> i64 {
        1+(self.top.z - self.bottom.z).abs()
    }

    fn width (&self) -> i64 {
        1+(self.top.y -self.bottom.y).abs()
    }
}


#[cfg(test)]
mod test {
    use crate::Cuboid;


    #[test]
    fn test_parse(){

        let all_cubes: Vec<Cuboid> = test_data().lines()
            .map(|z | {
                z.parse::<Cuboid>().unwrap()
            }).collect();

        all_cubes.iter().for_each(|x| println!("{:?}",x));

        
        assert_eq!( 3, all_cubes[0].width());
        assert_eq!( 3, all_cubes[0].length());
        assert_eq!( 3, all_cubes[0].depth());
        assert_eq!( 27, all_cubes[0].count_cubes());
    }

    fn test_data() -> &'static str{
        r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#
    }


}