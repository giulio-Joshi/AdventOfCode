use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_content = fs::read_to_string("day12/input.txt").expect("Can't find input");
    let paths = explore(&file_content);
    println!("Found valid paths {}", find_paths(&paths, 1));

    println!("Found valid paths (double small) {}", find_paths(&paths, 2));
}

pub fn find_paths(map: &[(String, String)], small_c_limit: usize) -> i32 {
    let mut all_paths: HashSet<String> = HashSet::new();
    all_paths.insert("start".into());

    let mut path_is_open = true;

    while path_is_open {
        let new_paths = map
            .iter()
            .fold(HashSet::new(), |mut new_paths: HashSet<String>, step| {
                all_paths.iter().for_each(|prev| {
                    if prev.ends_with("end") {
                        new_paths.insert(prev.into());
                        return;
                    }

                    if prev.ends_with(&step.1) {
                        try_step(&step.0, prev, small_c_limit)
                            .into_iter()
                            .for_each(|x| {
                                new_paths.insert(x);
                            });
                    } else if prev.ends_with(&step.0) {
                        try_step(&step.1, prev, small_c_limit)
                            .into_iter()
                            .for_each(|x| {
                                new_paths.insert(x);
                            });
                    }
                });

                new_paths
            });

        all_paths = new_paths;

        path_is_open = Option::is_some(&all_paths.iter().find(|&x| !x.ends_with("end")))
    }

    all_paths.len() as i32
}

fn try_step(end: &str, tracked_path: &str, small_c_limit: usize) -> Option<String> {
    if end.eq("start") {
        return None;
    }

    let dest_is_small = end.chars().next().unwrap().is_lowercase();

    let mut can_step_if_small =
        dest_is_small && tracked_path.matches(&end).count().lt(&small_c_limit);

    if small_c_limit > 1 {
        can_step_if_small &= small_caves_repeats_max(tracked_path, small_c_limit);
    }

    if !dest_is_small || can_step_if_small {
        let path_step = [tracked_path, ",", end];
        return Some(path_step.concat());
    }
    None
}

fn small_caves_repeats_max(path: &str, small_c_limit: usize) -> bool {
    let mut freq = HashMap::<String, i32>::new();
    let avoid = vec!["end", "start"];

    path.split(',')
        .filter(|&x| x.chars().next().unwrap().is_lowercase() && !avoid.contains(&x))
        .into_iter()
        .for_each(|x| {
            *freq.entry(String::from(x)).or_insert(0) += 1;
        });

    freq.into_iter()
        .filter(|v| v.1 as usize >= small_c_limit)
        .count()
        < small_c_limit
}

pub fn explore(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|z| {
            let mut parts = z.split('-');
            (parts.next().unwrap().into(), parts.next().unwrap().into())
        })
        .collect()
}

#[cfg(test)]
mod test {

    use crate::{explore, find_paths};

    #[test]
    fn test_find_paths() {
        let paths = explore(data());

        assert_eq!(19, find_paths(&paths, 1));
        assert_eq!(103, find_paths(&paths, 2));
    }

    fn data() -> &'static str {
        r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#
    }
}
