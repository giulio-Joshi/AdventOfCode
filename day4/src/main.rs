use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!("Hello, world!");
}


struct Board {
    numbers: Vec<Vec<i32>>,
    marked: Vec<(usize,usize)>
}

struct Game {
    winning: Vec<i32>,
    boards: Vec<Board>
}

impl Game {

    pub fn find_winner(&mut self) -> usize {
        for number in &self.winning {
            for b in &mut self.boards {

                if let Some(marked) = b.mark(number){
                    b.marked.push(marked);
                   
                }
            }

            for (i, win) in self.boards.iter().enumerate() {
                if win.is_winning() {
                    return i;
                }
            }
        }

        

        panic!("can't find winner");
    }

    
    
}

impl Board {


    pub fn mark(& self, number : &i32 ) -> Option<(usize,usize)> {

        self.numbers.iter().enumerate()
            .map_while( |k|  {
                    k.1.iter().position( |z| z.eq(number))
                        .map(|pos| ( k.0 , pos))})
            .next()
    
    }

    pub fn is_winning(& self ) -> bool {

        if self.marked.len() < 5 {   
            return false;
        }

        let board_size = self.numbers.len();

        for i in 0..board_size {
            if self.marked.iter().filter( |&x | x.0.eq(&i)).count() == board_size ||
                self.marked.iter().filter( |&y | y.1.eq(&i)).count() == board_size {
                return true;
            }
        }
        false    
    }


}


impl FromStr for Game {
    type Err = ParseIntError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {

        let winning_part = s.lines().next().unwrap();
        let board_part = s.lines().skip(2).collect::<Vec<&str>>();

        // Create every board, separated by an empty line, after removing the first 2 lines
        let board_part : Vec<Board> = board_part
                .rsplit(|&x|x.eq(""))
                .filter(|&x| !x.is_empty())
                .map( |x| x.join("\n").parse::<Board>().unwrap())
                .rev()
                .collect();

        Ok( Game{ winning: winning_part.split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            , boards: board_part } )
    }
}

impl FromStr for Board {
    type Err =ParseIntError;
    fn from_str(s : &str) -> Result<Self, Self::Err>{

        Ok(Board{ numbers:  s.lines().map(|x| x.split(' ')
                .filter( |&z| !z.is_empty())
            .map(|m| m.parse::<i32>().unwrap()).collect()).collect(),
            marked: vec!() })
    }
}



#[cfg(test)]
mod test {

    use crate::Board as Board;
    use crate::Game as Game;

    fn get_test_data() -> Game {
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#.parse().unwrap()

    }

    #[test]
    fn test_read_data(){

        let mut gameboard = get_test_data();

        assert_eq!( 7, gameboard.winning[0]);
        assert_eq!( 3, gameboard.boards.len());
        assert_eq!( 22, gameboard.boards[0].numbers[0][0]);

        assert_eq!( 2, gameboard.find_winner());

    }




}