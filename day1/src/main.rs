use std::fs;

/// Day 1, Puzzle one.
///
///  Note to self, after some peer-comparison: do a better study of iterators interfaces

fn main() {
    let scan_sweeps =
        fs::read_to_string("scan_sweeps.log").expect("Missing required file for input");

    let sweeps: Vec<i32> = scan_sweeps
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Found {} scan single increases", direct_scan_sweep(&sweeps));
    println!("Found {} scan 3fold increases", summatory_scans(&sweeps));
}

pub fn summatory_scans(sweeps: &[i32]) -> i32 {
    let mut increases: i32 = 0;
    let mut previous: Option<i32> = None;
    for (k, _) in sweeps.iter().enumerate() {
        if sweeps.len() > k + 2 {
            let current = sweeps[k] + sweeps[k + 1] + sweeps[k + 2];
            if let Some(prev) = previous {
                if current > prev {
                    increases += 1;
                }
            }
            previous = Some(current);
        }
    }
    increases
}

pub fn direct_scan_sweep(sweeps: &[i32]) -> i32 {
    let result = sweeps
        .iter()
        .fold((None, 0), |counter, item| match counter.0 {
            None => (Some(item), 0),
            Some(previous) => {
                let mut k: i32 = counter.1;
                if previous < item {
                    k += 1;
                }
                (Some(item), k)
            }
        });

    result.1
}
