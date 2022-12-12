// tests gets a bit verbose with this one, but that's not bad 👅
#[cfg(test)]
use std::println as info;

use pathfinding::prelude::astar;

pub fn execute(raw_input: &str) {
    let (start, end, map) = parse_setup(raw_input);

    println!("Character start at {start:?} , needs to go to {end:?}");
    println!();

    let steps = find_path(&start, &end, &map);

    println!(
        "It takes exactly {} steps to reach that point",
        steps.expect("Failed to calculate path").len() - 1
    );
    println!("\n====\n");

    println!("Looking for possible starting points... 🔎");

    let possible_starts = find_by_elevation(&map, &0);
    println!(
        "Found {} possible 🔻 (including original)",
        possible_starts.len()
    );

    let minimum_steps = find_minimum_path(&possible_starts, &end, &map);
    println!("Shortest path requires {minimum_steps}");
}

type ParsedMap = ((usize, usize), (usize, usize), Vec<Vec<u8>>);

// All possible directions
const MOVES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn successors(begin: &(usize, usize), map: &[Vec<u8>]) -> Vec<((usize, usize), u32)> {
    let mut he_rustic = vec![];

    MOVES.iter().for_each(|step| {
        let new_x: i32 = begin.0 as i32 + step.0;
        let new_y: i32 = begin.1 as i32 + step.1;
        if new_x >= 0 && new_x < map.len() as i32 && new_y >= 0 && new_y < map[0].len() as i32 {
            let difficulty = if map[begin.0][begin.1] < map[new_x as usize][new_y as usize] {
                (map[new_x as usize][new_y as usize] - map[begin.0][begin.1]) as u32
            } else {
                0_u32
            };

            // Add only available steps: costs 1.
            if difficulty <= 1 {
                he_rustic.push(((new_x as usize, new_y as usize), 1));
            }
        }
    });

    info!(
        "EXITS {begin:?} [{}] :🚪: {he_rustic:?}",
        map[begin.0][begin.1]
    );

    he_rustic
}

fn distance(point1: &(usize, usize), point2: &(usize, usize)) -> u32 {
    let calc_dist = point1.0.abs_diff(point2.0).pow(2) + point1.1.abs_diff(point1.1).pow(2);
    let distance: u32 = (calc_dist as f32).sqrt() as u32;
    info!(
        "Calc distance between {:?} and {:?} = {}",
        point1, point2, distance
    );
    distance
}

fn find_path(
    start: &(usize, usize),
    end: &(usize, usize),
    map: &[Vec<u8>],
) -> Option<Vec<(usize, usize)>> {
    if let Some(result) = astar(
        start,
        |p| successors(p, map),
        |p| distance(p, end),
        |p| p == end,
    ) {
        Some(result.0)
    } else {
        None
    }
}

fn find_minimum_path(
    possible_starts: &[(usize, usize)],
    end: &(usize, usize),
    map: &[Vec<u8>],
) -> usize {
    possible_starts
        .iter()
        .flat_map(|start_from| find_path(start_from, end, map))
        .map(|p| p.len() - 1)
        .min()
        .expect("Can't find minimal path?!?")
}

fn find_by_elevation(map: &[Vec<u8>], at_elevation: &u8) -> Vec<(usize, usize)> {
    let mut found = vec![];

    map.iter().enumerate().for_each(|(x, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &e)| e == *at_elevation)
            .for_each(|(y, _)| {
                found.push((x, y));
            })
    });

    found
}

fn parse_setup(raw: &str) -> ParsedMap {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = raw
        .lines()
        .enumerate()
        .map(|(x, r)| {
            r.chars()
                .enumerate()
                .map(|(y, point)| match point {
                    'E' => {
                        end = (x, y);
                        b'z' - b'a'
                    }
                    'S' => {
                        start = (x, y);
                        0
                    }
                    _ => point as u8 - b'a',
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    (start, end, map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_by_elevation() {
        let (_, end, map) = parse_setup(data());
        let possible_starts = find_by_elevation(&map, &0);
        assert_eq!(6, possible_starts.len());

        let minimum_found = find_minimum_path(&possible_starts, &end, &map);

        assert_eq!(29, minimum_found);
    }

    #[test]
    fn test_find_path() {
        let (start, end, map) = parse_setup(data());
        let steps = find_path(&start, &end, &map).unwrap();
        assert_eq!(31, steps.len() - 1);
    }

    #[test]
    fn test_successors() {
        let (start, _end, map) = parse_setup(data());
        let succ_result = successors(&start, &map);
        assert_eq!(1, succ_result[0].1);

        let succ_result = successors(&(4, 2), &map);
        println!("{succ_result:?}");
        assert_eq!(0, succ_result[0].1);
    }

    #[test]
    fn test_parser() {
        let (start, end, map) = parse_setup(data());
        assert_eq!((0, 0), start);
        assert_eq!((2, 5), end);
        assert_eq!(5, map.len());
    }

    fn data() -> &'static str {
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
    }
}
