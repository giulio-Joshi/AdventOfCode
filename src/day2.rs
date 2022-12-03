use core::panic;

pub fn execute(raw_input: &str) {
    let battles = read_battle(raw_input);
    let outcome = strategy_one( &battles );
    let second = strategy_two( raw_input);
    println!("Resulting outcome strategy {outcome}");
    println!("Resulting second strategy {second}");
}

#[derive(Copy, Clone,Debug)]
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Outcome {
    fn reverse(&self, other: &Sign ) ->u32 {

        match self {
            Self::Win => {
                6+ match other {
                    Sign::Rock => 2,
                    Sign::Paper => 3,
                    Sign::Scissor => 1,
                }
            },
            Self::Draw => {
                3 + match other {
                    Sign::Rock => 1,
                    Sign::Paper => 2,
                    Sign::Scissor => 3,
                }
            },
            Self::Lose => {
                match other {
                    Sign::Rock => 3,
                    Sign::Paper => 1,
                    Sign::Scissor => 2,
                }
            }
        }

    }
}


#[derive(Copy,Clone,Debug)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input  {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unexpected value")
        }
    }
}

impl Sign {

    fn value( &self ) -> u32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissor => 3
        }
    }

    fn score(&self, other: &Sign) -> u32 {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => 3,
                    Self::Paper => 0,
                    Self::Scissor => 6,
                }
            },
            Self::Paper => {
                match other {
                    Self::Rock => 6,
                    Self::Paper => 3,
                    Self::Scissor => 0,
                }
            },
            Self::Scissor => {
                match other {
                    Self::Rock => 0,
                    Self::Paper => 6,
                    Self::Scissor => 3,
                }
            }
        }
    }

}

impl From<&str> for Sign {
    fn from(input: &str) -> Self {
        match input  {
            "A" => Sign::Rock,
            "B" => Sign::Paper,
            "C" => Sign::Scissor,
            "X" => Sign::Rock,
            "Y" => Sign::Paper,
            "Z" => Sign::Scissor,
            _ => panic!("unexpected value")
        }
    }
}

fn strategy_two(input: &str) -> u32 {

    let battles : Vec<(Sign,Outcome)> = input.split('\n')
        .map(|v| {
            let stuff : Vec<&str> = v.split(' ').collect();
            (Sign::from(stuff[0]), Outcome::from(stuff[1]))
        }
        )
        .collect();

    guess_score( &battles)
}

fn guess_score(battles : &[(Sign, Outcome)] ) -> u32 {

    battles.iter()
        .map(|x| {
            x.1.reverse(&x.0)
        } )
        .sum()

}


fn read_battle(input: &str) -> Vec<(Sign, Sign)> {

    input.split('\n')
        .map(|v| {
            let stuff : Vec<Sign> = v.split(' ')
            .map(Sign::from)
            .collect();
            (stuff[0], stuff[1])
        }
        )
        .collect()
}

fn strategy_one( battles: &[(Sign,Sign)]) -> u32 {

    battles.iter().fold(0, |accum, elemen| {
        accum+ single_match_score(elemen)
    })
}

fn single_match_score( battle: &(Sign,Sign) ) -> u32 {

    battle.1.score(&battle.0) + battle.1.value()
}

#[cfg(test)]
mod test {

    use crate::day2::*;

    #[test]
    pub fn strategy_simple() {
        let data = data();
        let battles = read_battle(data);
        assert_eq!(3 , battles.len());

        assert_eq!(15, strategy_one( &battles ));
        assert_eq!(12, strategy_two( &data));
    }

    fn data() -> &'static str {
        r#"A Y
B X
C Z"#
    }
}
