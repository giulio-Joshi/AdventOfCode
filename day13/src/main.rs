use std::{fs, str::FromStr, num::ParseIntError, collections::HashSet};

use image::{RgbImage, ImageBuffer};

fn main() {
    let file_content = fs::read_to_string("day13/input.txt").expect("Can't find input");

    let mut stuff = convert(&file_content);

    println!("Found {} points and {} instructions", stuff.0.len(), stuff.1.len());

    let mut count= 0;

    for instr in stuff.1.into_iter() {

        count +=1;

    

        //println!("Folding {:?}", &instr);
        let temp = instr.apply( &stuff.0);
        
        stuff.0 = temp;
        if count == 1 {
    
            let filtered = filter_double( &stuff.0);
            println!("After folding : {}", filtered.len());
        }
    }


    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new( 50,50);


    for point  in stuff.0.into_iter() {

        img.put_pixel( point.x as u32, point.y as u32, image::Rgb( [255,255,255]));

    }

    img.save("day13/output.png").unwrap();
}


pub fn print_vect ( v: &[Point] ) -> String {


    v.into_iter( )
        .for_each(|x| {



        });



    String::from("")
}


pub fn filter_double( v : &[Point]) -> Vec<Point> {

    let mut list = HashSet::<Point>::new();
    v.iter().for_each( |p| { list.insert(*p); });
    list.into_iter().collect()
}


#[derive(Clone, Copy,Debug, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32
}


impl FromStr for Point {
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let coords : Vec<i32> = s.split(',')
            .take(2)
            .map(|z| z.parse::<i32>().unwrap() )
            .collect();

        Ok( Point{ x:coords[0] ,y:coords[1] } )
    }
}


#[derive(Debug)]
pub enum Instruction {
    FoldX(i32),
    FoldY(i32)
}

impl FromStr for Instruction {
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{

        let parts : Vec<&str> = s[11..].split('=').collect();

        if parts[0].eq("x") {

            return Ok( Instruction::FoldX( parts[1].parse().unwrap() ));
        } else if parts[0].eq("y") {

            return Ok( Instruction::FoldY( parts[1].parse().unwrap() ));
        }

        Err("".parse::<i32>().unwrap_err())
    }
}

impl Instruction {

    pub fn apply( &self, dots: &[Point] ) -> Vec<Point> {


        dots.into_iter().map( |point| {
            match self {

                &Instruction::FoldX(line)  =>{
                    if point.x > line {
                        Point { x : line-(point.x-line) , y: point.y}
                    } else {
                        *point
                    }
                },
                &Instruction::FoldY(line) => {

                    if point.y > line {
                        Point {x : point.x , y: line- (point.y-line) } 
                    } else {
                        *point
                    }
                }

            }
        }).collect()
    }

}

pub fn convert( data: &str )  -> (Vec<Point>,Vec<Instruction>){

    let le_points= data.lines()
        .into_iter()
        .take_while( |&x| !x.eq("") )
        .map( |d| d.parse::<Point>().unwrap() )
        .collect::<Vec<Point>>();
    
    let le_instructions = data.lines()
            .skip_while( |&x| !x.eq(""))
            .skip(1)
            .map( |v| v.parse::<Instruction>().unwrap() )
            .collect::<Vec<Instruction>>();

    (le_points, le_instructions)
}



#[cfg(test)]
mod test {
    use crate::{convert, filter_double};


    #[test]
    fn convert_test( ) {

        let mut stuff = convert(data());


        stuff.0.iter()
            .for_each(|x| println!("{:?}",&x));

        assert_eq!( 18 , stuff.0.len());
        assert_eq!( 2 , stuff.1.len());

        for instr in stuff.1.into_iter().take(1) {

            //println!("Folding {:?}", &instr);
            let temp = instr.apply( &stuff.0);

            temp.iter()
            .for_each(|x| println!("{:?}",&x));
            
            stuff.0 = temp;
        }

        let filtered = filter_double( &stuff.0);

        //println!(" Final == ");
        filtered.iter()
        .for_each(|x| println!("{:?}",&x));

//        println!("{:?}", &filtered);
        assert_eq!(17, filtered.len() );

    }


    fn data() -> &'static str {
        r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#
    }
}