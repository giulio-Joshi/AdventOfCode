use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("Can't find input");

    let fishes: Vec<usize> = file_content
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let after_80 = forward_time(&fishes, 80);
    println!(" After 80 days, you got {} fishes", after_80);
    let after_256 = forward_time(&fishes, 256);
    println!(" After 256 days, you got {} fishes", after_256);
}

fn forward_time(starting_fishes: &[usize], days: u64) -> u64 {
    const POSSIBLE_TIMERS: usize = 9;

    let mut fish_timers: [u64; POSSIBLE_TIMERS] = [0; POSSIBLE_TIMERS];

    for &fish in starting_fishes.iter() {
        fish_timers[fish] += 1;
    }

    for _ in 0..days {
        let spawning = fish_timers[0];
        for clock in 1..9 {
            if fish_timers[clock] > 0 {
                let fish_ageing = fish_timers[clock];
                fish_timers[clock - 1] += fish_ageing;
                fish_timers[clock] -= fish_ageing;
            }
        }

        fish_timers[6] += spawning;
        fish_timers[8] += spawning;
        fish_timers[0] -= spawning;
    }

    fish_timers.into_iter().sum()
}
#[cfg(test)]
mod test {

    use crate::forward_time;

    #[test]
    fn count_fishes_01() {
        let fishes: Vec<usize> = read_data();

        assert_eq!(5934, forward_time(&fishes, 80));
        assert_eq!(26984457539, forward_time(&fishes, 256));
    }

    fn read_data() -> Vec<usize> {
        String::from("3,4,3,1,2")
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }
}
