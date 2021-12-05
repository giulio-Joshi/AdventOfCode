use std::{fs, str::FromStr, num::ParseIntError};

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let amount:i32 = parts[1].parse()?;

        if parts[0] == "forward" {
            return Ok(Direction::Forward(amount));
        } else if parts[0] == "up" {
            return Ok(Direction::Up(amount));
        } else if parts[0] == "down" {
            return Ok(Direction::Down(amount));
        }

        Err("".parse::<i32>().unwrap_err())
    }
}

#[derive(Clone,Copy)]
struct Position {
    horizon:i32,
    depth:i32,
    aim: i32
}

impl Position {
    pub fn new() -> Self {
        Position { horizon: 0 , depth:0 , aim: 0}
    }

    pub fn calc_position(self) -> i32 {
        self.horizon * self.depth
    }

    pub fn add_direction(mut self, dir: Direction) -> Self {
        match dir {
            Direction::Up(value) => {
                // Note: used in first part
                //self.depth-= value;
                self.aim-=value;
            },
            Direction::Down(value) => {
                // Note: used in first part
                //self.depth+=value;
                self.aim+=value;
            },
            Direction::Forward(value) => {
                self.horizon+=value;
                self.depth+= self.aim*value;
            }
        }
        self
    }
}

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Can't find input");

    let final_pos = file_content.split('\n')
        .map( |x| x.parse::<Direction>().unwrap() )
        .fold(Position::new(), | pos ,dir|{
        pos.add_direction(dir)
    });

    println!("Final position is {} ", final_pos.calc_position());
}


#[cfg(test)]
mod tests {
    use crate::{Position, Direction};

    #[test]
    fn simple_steps(){

        let directions = vec!( Direction::Forward(5), 
            Direction::Down(5), 
            Direction::Forward(8), 
            Direction::Up(3), 
            Direction::Down(8), 
            Direction::Forward(2));

        let final_pos=  directions.into_iter().fold(Position::new(),
        |pos,x|{
            pos.add_direction(x)
        });

        assert_eq!( 900, final_pos.calc_position());
    }


}