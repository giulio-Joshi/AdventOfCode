use std::fs;
fn main() {
    let file_content = fs::read_to_string("day9/input.txt").expect("Can't find input");
    let result = find_low(&convert(&file_content));
    let total: u32 = result.into_iter().map(|z| z as u32 + 1_u32).sum();
    println!("Solution: {}", total);
}

fn find_low(points: &Vec<Vec<char>>) -> Vec<u8> {
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

fn find_basin(points: &Vec<Vec<char>>) -> Vec<u8> {
    let mut low_points: Vec<(usize, usize)> = vec![];

    for x in 0..points.len() {
        let mut is_head = None;
        let mut slice_start = 0;
        let mut slice_end = x + 2;

        if x == (points.len() - 1) {
            slice_end = x + 1;
            is_head = Some(false);
        } else if x == 0 {
            is_head = Some(true);
        } else {
            slice_start = x - 1;
        }

        for y in 0..points[x].len() {
            if is_less_in_group(points[x][y], x, &points[slice_start..slice_end], is_head) {
                low_points.push((x, y));
            }
        }
    }

    println!("{:?}", low_points);

    vec![]
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
    use crate::{convert, find_basin, find_low};

    #[test]
    fn test_0() {
        let expected = vec![1, 0, 5, 5];

        let smoke = convert(data());

        let found = find_low(&smoke);

        assert_eq!(expected, found);
        let total: u32 = found.into_iter().map(|z| z as u32 + 1_u32).sum();
        assert_eq!(15, total);

        let basin_total = find_basin(&smoke)
            .into_iter()
            .map(|z| z as i32 + 1_i32)
            .product();
        assert_eq!(1134, basin_total);
    }

    fn data() -> &'static str {
        r#"2199943210
3987894921
9856789892
8767896789
9899965678"#
    }
}
