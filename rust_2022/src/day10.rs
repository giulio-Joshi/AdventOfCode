use std::{
    collections::HashMap,
    ops::{Add, RangeInclusive},
};

pub fn execute(raw_input: &str) {
    let values = find_values_at_cycle(raw_input, &[20, 60, 100, 140, 180, 220]);
    let total: i32 = values.iter().map(|k| k.1).sum();
    println!("Values : {total} ");
    println!();

    println!("Crt Shows:\n{}", instructions_to_crt(raw_input));
    println!();
}

fn instructions_to_crt(program: &str) -> String {
    let mut registry: i32 = 1;
    let mut cycle: usize = 0;
    let mut crt_view: Vec<bool> = Vec::with_capacity(250);

    for l in program.lines() {
        cycle += 1;
        draw_pixel(registry, cycle, &mut crt_view);

        if !l.eq("noop") {
            let value = instruction_to_value(l);
            cycle += 1;
            draw_pixel(registry, cycle, &mut crt_view);
            registry += value;
        }
    }

    crt_view
        .chunks(40)
        .map(|p| {
            let mut s = p
                .iter()
                .map(|&e| if e { '#' } else { '.' })
                .collect::<String>();
            s.push('\n');
            s
        })
        .collect::<String>()
}

fn draw_pixel(registry: i32, cycle: usize, crt_view: &mut Vec<bool>) {
    let sprite_position = registry - 1..=registry + 1;

    let drawing_sprite = ((cycle - 1) % 40) as i32;

    if sprite_position.contains(&drawing_sprite) {
        crt_view.push(true);
    } else {
        crt_view.push(false);
    }
}

fn find_values_at_cycle(program: &str, cycle_n: &[usize]) -> HashMap<usize, i32> {
    let mut registered = HashMap::new();
    let mut registry: i32 = 1;
    let mut cycle: usize = 1;

    let mut interest = cycle_n.iter();
    let mut current_interest = interest.next().unwrap();

    for l in program.lines() {
        cycle += 1;
        if *current_interest == cycle {
            registered.insert(cycle, registry * cycle as i32);
            if let Some(ii) = interest.next() {
                current_interest = ii;
            } else {
                break;
            }
        }

        if !l.eq("noop") {
            let value = instruction_to_value(l);
            registry += value;
            cycle += 1;
            if *current_interest == cycle {
                registered.insert(cycle, registry * cycle as i32);
                if let Some(ii) = interest.next() {
                    current_interest = ii;
                } else {
                    break;
                }
            }
        }
    }

    registered
}

fn instruction_to_value(l: &str) -> i32 {
    l.split(' ')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_view() {
        let crt_shows = instructions_to_crt(data());
        assert_eq!(expected_result(), crt_shows);
    }

    #[test]
    fn test_find_values_at_cycle() {
        let values = find_values_at_cycle(data(), &[20, 60, 100, 140, 180, 220]);
        assert_eq!(420, values[&20]);
        assert_eq!(1140, values[&60]);
        assert_eq!(1800, values[&100]);
        assert_eq!(2940, values[&140]);
        assert_eq!(2880, values[&180]);
        assert_eq!(3960, values[&220]);
    }

    fn expected_result() -> &'static str {
        r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
    }

    fn data() -> &'static str {
        r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
    }
}
