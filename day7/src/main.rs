use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("Can't find input");

    let crabs = file_content
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!(
        "Fuel spent to reach pos : {}",
        calc_best(&crabs, Method::Standard)
    );
    println!(
        "Fuel spent to reach pos, 2nd row : {}",
        calc_best(&crabs, Method::NthTriangle)
    );
}

enum Method {
    Standard,
    NthTriangle,
}

fn calc_best(crabs: &[i32], method: Method) -> i32 {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let fuel_calc = match method {
        Method::Standard => |target: i32, position: i32| (target - position).abs(),
        Method::NthTriangle => |target: i32, position: i32| {
            let n = (target - position).abs();
            (n.pow(2) + n) / 2
        },
    };

    (min..max)
        .into_iter()
        .map(|target_pos| {
            crabs
                .iter()
                .map(|&crab_pos| fuel_calc(target_pos, crab_pos))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{calc_best, Method};

    #[test]
    fn test_fuel() {
        let positions: Vec<i32> = String::from("16,1,2,0,4,2,7,1,2,14")
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        assert_eq!(37, calc_best(&positions, Method::Standard));
        assert_eq!(168, calc_best(&positions, Method::NthTriangle));
    }
}
