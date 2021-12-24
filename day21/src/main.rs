fn main() {
    let finished_game = GameResult::brute_force_game( 8,4,1000,1);
    println!("{:?}", finished_game);
    println!("Your guess would be: {}", finished_game.losing_score() * finished_game.rolls);
}

#[derive(Debug)]
struct GameResult {
    player_position: Vec<usize>,
    player_score: Vec<usize>,
    rolls: usize
}

impl GameResult{


    fn brute_force_game( p1_start: usize, p2_start: usize, winning: usize , starting_roll : usize)  -> Self {

        let mut game = GameResult { player_position : vec!(p1_start, p2_start) , player_score: vec!(0,0), rolls: starting_roll};

        while game.player_score.iter().max().unwrap() < &winning  {

            game.player_position[0]= calc_pos_score( game.player_position[0], calc_step(game.rolls, 100, 3));
            game.player_score[0]+= game.player_position[0];
            game.rolls+=3;

            if game.player_score[0] < winning {
                let go = calc_pos_score( game.player_position[1], calc_step(game.rolls, 100, 3));
                game.player_position[1]= go;
                game.player_score[1]+= game.player_position[1];
                game.rolls+=3;
            }
        }
        game.rolls-=starting_roll;
        game
    }

    fn losing_score( &self) -> usize {
        *self.player_score.iter().min().unwrap()
    }
}

fn calc_step( start_value: usize, dice_size: usize, rolls: usize) -> usize {

    let mut sum =0;
    for i in start_value..start_value+rolls {
        if  i > dice_size {
            sum += i-dice_size;
        }else{
            sum += i;
        }
    } 
    sum
}


fn calc_pos_score(player_pos: usize, advance: usize) -> usize{
    let score = (player_pos+advance)%10;
    if score == 0 {
        10
    } else {
        score
    }
}



#[cfg(test)]
mod test {
    use crate::{GameResult, calc_pos_score, calc_step};


    #[test]
    fn winning_diract( ) {
        GameResult::dirac_win(4,8,21);

        assert_eq!(15,calc_step(4, 100, 3));
    }

    #[test]
    fn brute_force( ){

        let finished_game = GameResult::brute_force_game( 4,8, 1000, 1);

        assert_eq!(745, finished_game.player_score[1]);
        assert_eq!(993, finished_game.rolls);
    }

    #[test]
    fn steps(){

        assert_eq!(6,calc_step(1, 100, 3));
        assert_eq!(200,calc_step(99, 100, 3));
        assert_eq!(15,calc_step(4, 100, 3));

        assert_eq!( 3, calc_pos_score(8,15));
        assert_eq!( 2, calc_pos_score(1,1));
    }

    #[test]
    fn step_forward() {
        assert_eq!(10 , calc_pos_score(4, 6));

    }

}