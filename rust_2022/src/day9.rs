use std::collections::HashSet;

pub fn execute(raw_input: &str) {
    let head_positions = to_head_movements(raw_input);
    let tail_positions = follow_tail_positions(&head_positions);

    let single_steps = count_single_steps(&tail_positions);

    // Must add 1 for initial position (which is not counted)
    println!("Found {} steps for tail", single_steps);
    println!();
    let all_tails = multiple_tails(&head_positions, 9);

    println!(
        "Found {} steps for 9 tails",
        count_single_steps(&all_tails[8])
    );
    println!();
}

fn multiple_tails(input: &[(i16, i16)], tails: usize) -> Vec<Vec<(i16, i16)>> {
    let mut all_tails = Vec::with_capacity(tails);
    for i in 0..tails {
        if i == 0 {
            all_tails.push(follow_tail_positions(input));
        } else {
            let new_tail = follow_tail_positions(&all_tails[i - 1]);
            all_tails.push(new_tail);
        }
    }
    all_tails
}

fn follow_tail_positions(input: &[(i16, i16)]) -> Vec<(i16, i16)> {
    let mut current_position: (i16, i16) = (0, 0);
    input
        .iter()
        .flat_map(|&head| {
            let diff_pos = (current_position.0 - head.0, current_position.1 - head.1);

            if diff_pos.1.abs() > 1 || diff_pos.0.abs() > 1 {
                if diff_pos.0 != 0 {
                    current_position.0 -= diff_pos.0 / diff_pos.0.abs();
                }

                if diff_pos.1 != 0 {
                    current_position.1 -= diff_pos.1 / diff_pos.1.abs();
                }

                return Some(current_position);
            }
            None
        })
        .collect()
}

fn count_single_steps(positions: &[(i16, i16)]) -> usize {
    let mut single_pos: HashSet<(i16, i16)> = HashSet::new();

    positions.iter().for_each(|&p| {
        single_pos.insert(p);
    });

    single_pos.len() + 1
}

fn to_head_movements(input: &str) -> Vec<(i16, i16)> {
    let mut position = (0, 0);

    input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .flat_map(|dir| match dir[0] {
            "U" => (0..(dir[1].parse::<u16>().unwrap()))
                .map(|_| {
                    position.1 -= 1;
                    (position.0, position.1)
                })
                .collect::<Vec<(i16, i16)>>(),
            "L" => (0..(dir[1].parse::<u16>().unwrap()))
                .map(|_| {
                    position.0 -= 1;
                    (position.0, position.1)
                })
                .collect::<Vec<(i16, i16)>>(),
            "R" => (0..(dir[1].parse::<u16>().unwrap()))
                .map(|_| {
                    position.0 += 1;
                    (position.0, position.1)
                })
                .collect::<Vec<(i16, i16)>>(),
            "D" => (0..(dir[1].parse::<u16>().unwrap()))
                .map(|_| {
                    position.1 += 1;
                    (position.0, position.1)
                })
                .collect::<Vec<(i16, i16)>>(),
            _ => {
                panic!("Unrecognized direction {}", dir[0]);
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiple_tails() {
        let head_positions = to_head_movements(data_two());
        let all_tails = multiple_tails(&head_positions, 9);
        assert_eq!(36, count_single_steps(&all_tails[8]));
    }

    #[test]
    fn get_positions() {
        let head_positions = to_head_movements(data());
        assert_eq!(24, head_positions.len());
        let tail_positions = follow_tail_positions(&head_positions);
        let single_steps = count_single_steps(&tail_positions);
        assert_eq!(13, single_steps);
    }

    fn data() -> &'static str {
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#
    }

    fn data_two() -> &'static str {
        r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#
    }
}
