use std::collections::HashSet;

pub fn execute(raw_input: &str) {
    let matrix = to_matrix(raw_input);
    println!("Found {} visible trees", find_visible(&matrix));
    println!("Best scenic score =  {} ", find_best_scenic_score(&matrix));
    println!()
}

fn find_best_scenic_score(matrix: &[Vec<u8>]) -> usize {
    let mut best_scenic = 0;
    matrix.iter().enumerate().for_each(|(x, tree_row)| {
        tree_row.iter().enumerate().for_each(|(y, tree)| {
            // Avoid the edges
            if y == 0 || y == matrix[0].len() - 1 || x == 0 || x == matrix.len() - 1 {
                return;
            }
            let score = find_score(matrix, tree, &(x, y));
            if score > best_scenic {
                best_scenic = score;
            }
        })
    });

    best_scenic
}

fn find_score(matrix: &[Vec<u8>], &value: &u8, position: &(usize, usize)) -> usize {
    let range_left = 0..position.0;
    let range_right = position.0 + 1..matrix.len();
    let range_up = 0..position.1;
    let range_down = position.1 + 1..matrix[0].len();

    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_up = 0;
    let mut score_down = 0;

    // Slow and repetitive

    for x in range_left.rev() {
        score_left += 1;
        if matrix[x][position.1] >= value {
            break;
        }
    }
    for x in range_right {
        score_right += 1;
        if matrix[x][position.1] >= value {
            break;
        }
    }

    for y in range_up.rev() {
        score_up += 1;
        if matrix[position.0][y] >= value {
            break;
        }
    }

    for y in range_down {
        score_down += 1;
        if matrix[position.0][y] >= value {
            break;
        }
    }
    score_left * score_right * score_up * score_down
}

fn find_internal_coordinates(matrix: &[Vec<u8>]) -> HashSet<(usize, usize)> {
    let mut internal_coordinates: HashSet<(usize, usize)> = HashSet::new();

    matrix.iter().enumerate().for_each(|x| {
        let mut bigger = None;
        x.1.iter().enumerate().for_each(|y| {
            if let Some(size) = bigger {
                if y.1 > &size {
                    bigger = Some(*y.1);
                    internal_coordinates.insert((x.0, y.0));
                }
            } else {
                bigger = Some(*y.1);
                internal_coordinates.insert((x.0, y.0));
            }
        });

        let mut bigger = None;
        x.1.iter().enumerate().rev().for_each(|y| {
            if let Some(size) = bigger {
                if y.1 > &size {
                    bigger = Some(*y.1);
                    internal_coordinates.insert((x.0, y.0));
                }
            } else {
                bigger = Some(*y.1);
                internal_coordinates.insert((x.0, y.0));
            }
        });
    });

    for (y, _) in matrix[0].iter().enumerate() {
        let mut bigger = None;
        matrix.iter().enumerate().for_each(|(x, _)| {
            if let Some(size) = bigger {
                if matrix[x][y] > size {
                    bigger = Some(matrix[x][y]);
                    internal_coordinates.insert((x, y));
                }
            } else {
                bigger = Some(matrix[x][y]);
                internal_coordinates.insert((x, y));
            }
        });

        let mut bigger = None;
        matrix.iter().enumerate().rev().for_each(|(x, _)| {
            if let Some(size) = bigger {
                if matrix[x][y] > size {
                    bigger = Some(matrix[x][y]);
                    internal_coordinates.insert((x, y));
                }
            } else {
                bigger = Some(matrix[x][y]);
                internal_coordinates.insert((x, y));
            }
        });
    }
    internal_coordinates
}

fn find_visible(matrix: &[Vec<u8>]) -> usize {
    find_internal_coordinates(matrix).len()
}

fn to_matrix(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .flat_map(|c| c.to_string().parse::<u8>())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day8::find_best_scenic_score;

    use super::find_visible;
    use super::to_matrix;

    #[test]
    fn test_find_best_scenic_score() {
        let matrix = to_matrix(data());
        assert_eq!(8, find_best_scenic_score(&matrix));
    }

    #[test]
    fn find_visible_trees() {
        let matrix = to_matrix(data());
        assert_eq!(21, find_visible(&matrix));
    }

    #[test]
    fn test_to_matrix() {
        let matrix = to_matrix(data());
        assert_eq!(5, matrix.len());
        assert_eq!(5, matrix[0].len());
    }

    fn data() -> &'static str {
        r#"30373
25512
65332
33549
35390"#
    }
}
