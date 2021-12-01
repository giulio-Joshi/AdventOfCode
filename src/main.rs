use core::panic;
use std::fs;

fn main() {

    let scan_sweeps = match fs::read_to_string("scan_sweeps.log") {
            Ok(i)  => i,
            Err(_)  => panic!("Failed to read scan_sweeps.log"),

    };

    println!("Found {} scan single increases", direct_scan_sweep(&scan_sweeps));
    println!("Found {} scan 3fold increases", summatory_scans(&scan_sweeps));
}


pub fn summatory_scans( sweeps: &str ) -> i32 {

    
    let splits: Vec<i32>= sweeps.split('\n')
        .map( |e| e.parse::<i32>().unwrap())
        .collect();

    let mut increases :i32 = 0;
    let mut previous :Option<i32> = None;
    for (k,_) in splits.iter().enumerate() {

        if splits.len() > k+2 {
            let current = splits[k] + splits[k+1] + splits[k+2];
            if let Some(prev) = previous {
                if current > prev {
                    increases+=1;
                }
            } 
            previous = Some(current);
        }
    }
    increases
}


pub fn direct_scan_sweep( sweeps : &str  ) -> i32 {

    let result= sweeps.split('\n')
    .map( |e| e.parse::<i32>().unwrap())
        .fold( (None, 0), | counter, item|{
            match counter.0  {
                None => {
                    (Some(item), 0)
                },
                Some(previous) => {
                    let mut k : i32 = counter.1;
                    if previous < item {
                        k+=1;
                    }
                    ( Some(item), k)
                }
            }
        });

    result.1
}
