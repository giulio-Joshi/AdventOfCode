use core::panic;

pub fn execute(raw_input: &str) {
    let battles = read_battle(raw_input);
    let outcome = strategy_one( &battles );
    println!("Resulting outcome strategy {outcome}");
}

#[derive(Copy,Clone)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

impl Sign {
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

impl From<String> for Sign {
    fn from(input: String) -> Self {
        match input.as_str()  {
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


fn read_battle(input: &str) -> Vec<(Sign, Sign)> {

    input.split('\n')
        .map(|v| {
            let stuff : Vec<Sign> = v.split(' ')
            .map( |s| Sign::from(s.to_owned()))
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

    battle.1.score(&battle.0) + match &battle.1 {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissor => 3
    }
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
    }

    fn data() -> &'static str {
        r#"A Y
B X
C Z"#
    }
}
