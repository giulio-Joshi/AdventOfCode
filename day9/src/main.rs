use std::{collections::HashSet, fs};
fn main() {
    let file_content = fs::read_to_string("day9/input.txt").expect("Can't find input");
    let result = find_low(&convert(&file_content));
    let total: u32 = result.into_iter().map(|z| z as u32 + 1_u32).sum();

    println!("Solution: {}", total);

    let mut basin_total = find_basins(&convert(&file_content));
    basin_total.sort_by(|a, b| b.cmp(a));
    println!(
        "Solution basins: {}",
        basin_total.iter().take(3).product::<i64>()
    );
}

fn find_low(points: &[Vec<char>]) -> Vec<u8> {
    let mut all_lows: Vec<char> = filter_slice(&points[0..2], Some(true));

    all_lows.append(
        &mut points
            .windows(3)
            .flat_map(|m| filter_slice(m, None))
            .collect(),
    );

    all_lows.append(&mut filter_slice(
        &points[points.len() - 2..points.len()],
        Some(false),
    ));

    all_lows.into_iter().map(|x| x as u8 - b'0').collect()
}

fn find_basins(points: &[Vec<char>]) -> Vec<i64> {
    let mut basins: Vec<HashSet<(usize, usize)>> = vec![];

    for x in 0..points.len() {
        for y in 0..points[x].len() {
            if !points[x][y].eq(&'9') {
                assign_to_basin(&x, &y, &mut basins);
            }
        }
    }

    while let Some(mut merge) = merge_basins(&basins) {
        //println!("Basin {} and {} are contigous", merge.0, merge.1);

        let travasa = basins.remove(merge.0);
        if merge.1 > merge.0 {
            merge.1 -= 1;
        }

        travasa.into_iter().for_each(|copy| {
            basins.get_mut(merge.1).unwrap().insert(copy);
        });
    }

    //println!("Found basins:\n\n");

    basins.iter().map(|k| k.len() as i64).collect()
}

fn assign_to_basin(x: &usize, y: &usize, basins: &mut Vec<HashSet<(usize, usize)>>) {
    (0..basins.len()).for_each(|basin_idx| {
        if let Some(_nearest) = basins[basin_idx]
            .iter()
            .find(|&content| points_touch(&(*x, *y), content))
        {
            basins[basin_idx].insert((*x, *y));
            return;
        }
    });

    let mut new_basin: HashSet<(usize, usize)> = HashSet::new();
    new_basin.insert((*x, *y));
    basins.push(new_basin);
}

/// Will merge touching basins
fn merge_basins(points: &[HashSet<(usize, usize)>]) -> Option<(usize, usize)> {
    for x in 0..points.len() {
        for t in 0..points.len() {
            if t == x {
                continue;
            }
            if points[x]
                .iter()
                .find(|&other| points[t].iter().any(|inner| points_touch(other, inner)))
                .is_some()
            {
                return Some((x, t));
            }
        }
    }

    None
}

fn points_touch(point: &(usize, usize), other: &(usize, usize)) -> bool {
    let mut is_near: bool = false;

    if point.0 > 0 {
        is_near |= other.0 == point.0 - 1 && other.1 == point.1;
    }
    if point.1 > 0 {
        is_near |= other.0 == point.0 && other.1 == point.1 - 1;
    }

    is_near |= other.0 == point.0 + 1 && other.1 == point.1;
    is_near |= other.0 == point.0 && other.1 == point.1 + 1;

    is_near
}

fn filter_slice(x: &[Vec<char>], head: Option<bool>) -> Vec<char> {
    let mut all_results = vec![];
    let mut relative_row = 1;

    if let Some(is_head) = head {
        if is_head {
            relative_row = 0;
        } else {
            relative_row = 1;
        }
    }

    for i in 0..x[0].len() {
        if is_less_in_group(x[relative_row][i], i, x, head) {
            all_results.push(x[relative_row][i]);
        }
    }
    all_results
}

fn convert(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|z| z.chars().collect::<Vec<char>>())
        .collect()
}

fn is_less_in_group(value: char, index: usize, group: &[Vec<char>], head: Option<bool>) -> bool {
    let mut is_less_in_group = true;
    let mut relative_row = 1;

    // head/tail/central part
    match head {
        Some(true) => {
            relative_row = 0;
            is_less_in_group &= value < group[1][index];
        }
        Some(false) => {
            relative_row = 1;
            is_less_in_group &= value < group[0][index];
        }
        None => {
            is_less_in_group &= value < group[0][index] && value < group[2][index];
        }
    }

    // left boundary
    if index > 0 {
        is_less_in_group &= value < group[relative_row][index - 1];
    }

    // right boundary
    if index < group[0].len() - 1 {
        is_less_in_group &= value < group[relative_row][index + 1];
    }

    is_less_in_group
}

#[cfg(test)]
mod test {
    use crate::{convert, find_basins, find_low};

    #[test]
    fn test_0() {
        let expected = vec![1, 0, 5, 5];

        let smoke = convert(data());

        let found = find_low(&smoke);

        assert_eq!(expected, found);
        let total: u32 = found.into_iter().map(|z| z as u32 + 1_u32).sum();
        assert_eq!(15, total);

        let mut basin_total = find_basins(&smoke);

        basin_total.sort_by(|a, b| b.cmp(a));

        assert_eq!(1134, basin_total.iter().take(3).product::<i64>());
    }

    fn data() -> &'static str {
        r#"2199943210
3987894921
9856789892
8767896789
9899965678"#
    }
}
