use std::collections::HashSet;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("day11/input.txt").expect("Can't find input");
    let mut octo = matrix_to_output(&file_content);

    println!(
        "Found {} flashes in 100 steps",
        calc_flashes(&mut octo, Some(100))
    );
    let mut octo = matrix_to_output(&file_content);
    println!(
        "Found FULL flashes in {} steps",
        calc_flashes(&mut octo, None)
    );
}

pub fn matrix_to_output(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|z| z.chars().collect::<Vec<char>>())
        .map(|c| c.iter().map(|z| *z as u8 - b'0').collect())
        .collect::<Vec<Vec<u8>>>()
}

pub fn calc_flashes(octo: &mut Vec<Vec<u8>>, steps: Option<usize>) -> usize {
    let mut actual_flashes: usize = 0;

    let mut step_max = usize::MAX;
    let mut block_at_simult = true;

    if let Some(expressed) = steps {
        step_max = expressed;
        block_at_simult = false;
    }

    for step in 0..step_max {
        let mut flashes_points: HashSet<(usize, usize)> = HashSet::new();
        let mut flashes_locs: Vec<(usize, usize)> = vec![];

        // increase value
        octo.iter_mut().enumerate().for_each(|x| {
            x.1.iter_mut().enumerate().for_each(|y| {
                *y.1 += 1;
                if *y.1 > 9 && flashes_points.insert((x.0, y.0)) {
                    splash_in_range(x.0 as i32, y.0 as i32)
                        .into_iter()
                        .for_each(|flash| {
                            flashes_locs.push(flash);
                        });
                }
            });
        });

        while !flashes_locs.is_empty() {
            let loc = flashes_locs.pop().unwrap();
            octo[loc.0][loc.1] += 1;
            if octo[loc.0][loc.1] > 9 && flashes_points.insert((loc.0, loc.1)) {
                splash_in_range(loc.0 as i32, loc.1 as i32)
                    .into_iter()
                    .for_each(|flash| {
                        flashes_locs.push(flash);
                    });
            }
        }

        // Stops when all the octopus flash togheter
        if block_at_simult && flashes_points.len() == 100 {
            return step + 1;
        }

        // restore energy
        flashes_points.iter().for_each(|&x| {
            octo[x.0][x.1] = 0;
        });

        actual_flashes += flashes_points.len();
    }

    actual_flashes as usize
}

fn splash_in_range(x: i32, y: i32) -> Vec<(usize, usize)> {
    let mut flash_locs: Vec<(usize, usize)> = vec![];

    for xx in (x - 1)..(x + 2) {
        for yy in (y - 1)..(y + 2) {
            if (0..10).contains(&xx) && (0..10).contains(&yy) && !(xx, yy).eq(&(x, y)) {
                flash_locs.push((xx as usize, yy as usize));
            }
        }
    }
    flash_locs
}

#[cfg(test)]
mod test {
    use crate::{calc_flashes, matrix_to_output};

    #[test]
    fn part_01() {
        let mut octo = matrix_to_output(data());

        assert_eq!(1656, calc_flashes(&mut octo, Some(100)));
    }

    #[test]
    fn part_02() {
        let mut octo = matrix_to_output(data());

        assert_eq!(195, calc_flashes(&mut octo, None));
    }

    fn data() -> &'static str {
        r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#
    }
}
