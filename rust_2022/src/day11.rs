use std::collections::VecDeque;

pub fn execute() {
    // I'm tired about making parsers

    let stacks = vec![
        VecDeque::from([93, 98]),
        VecDeque::from([95, 72, 98, 82, 86]),
        VecDeque::from([85, 62, 82, 86, 70, 65, 83, 76]),
        VecDeque::from([86, 70, 71, 56]),
        VecDeque::from([77, 71, 86, 52, 81, 67]),
        VecDeque::from([89, 87, 60, 78, 54, 77, 98]),
        VecDeque::from([69, 65, 63]),
        VecDeque::from([89]),
    ];
    let monkes = vec![
        Monke {
            operation: MonkeOperation::Multiply(17),
            test_div: 19,
            monke_route: [5, 3],
        },
        Monke {
            operation: MonkeOperation::Sum(5),
            test_div: 13,
            monke_route: [7, 6],
        },
        Monke {
            operation: MonkeOperation::Sum(8),
            test_div: 5,
            monke_route: [3, 0],
        },
        Monke {
            operation: MonkeOperation::Sum(1),
            test_div: 7,
            monke_route: [4, 5],
        },
        Monke {
            operation: MonkeOperation::Sum(4),
            test_div: 17,
            monke_route: [1, 6],
        },
        Monke {
            operation: MonkeOperation::Multiply(7),
            test_div: 2,
            monke_route: [1, 4],
        },
        Monke {
            operation: MonkeOperation::Sum(6),
            test_div: 3,
            monke_route: [7, 2],
        },
        Monke {
            operation: MonkeOperation::Power,
            test_div: 11,
            monke_route: [0, 2],
        },
    ];

    let mut touched_items = turn_execute(stacks.clone(), &monkes, 20, &StressRelief::Classic(3));

    println!("Got all values {touched_items:?}");

    touched_items.sort_by(|a, b| b.cmp(a));

    println!("After sorting: ");
    println!("1st {}", &touched_items[0]);
    println!("2nd {}", &touched_items[1]);
    println!("Got = {} ", touched_items[1] * touched_items[0]);

    println!("\n====\nNew 10000 rounds\n====\n");

    let stress_relief_factor = monkes.iter().map(|m| m.test_div).product();

    let mut touched_items = turn_execute(
        stacks,
        &monkes,
        10000,
        &StressRelief::Divisor(stress_relief_factor),
    );

    println!("Got all values {touched_items:?}");

    touched_items.sort_by(|a, b| b.cmp(a));

    println!("After sorting: ");
    println!("1st {}", &touched_items[0]);
    println!("2nd {}", &touched_items[1]);
    println!("Got = {} ", touched_items[1] * touched_items[0]);
}

enum MonkeOperation {
    Multiply(u64),
    Sum(u64),
    Power,
}

struct Monke {
    operation: MonkeOperation,
    test_div: u64,
    //
    monke_route: [usize; 2],
}

enum StressRelief {
    Classic(u64),
    Divisor(u64),
}

impl Monke {
    fn apply_operation(&self, item: u64) -> u64 {
        use MonkeOperation::*;
        match self.operation {
            Multiply(x) => item * x,
            Sum(x) => item + x,
            Power => item * item,
        }
    }

    fn throws_to_first(&self, item: &mut u64, worry_level: &StressRelief) -> bool {
        match worry_level {
            StressRelief::Classic(divide) => {
                *item /= divide;
            }
            StressRelief::Divisor(divisor) => {
                *item %= divisor;
            }
        }
        (*item % self.test_div) == 0
    }
}

fn turn_execute(
    mut stacks: Vec<VecDeque<u64>>,
    monkes: &[Monke],
    turns: usize,
    worry: &StressRelief,
) -> Vec<usize> {
    let mut count_inspections = vec![0; monkes.len()];

    for _turn in 0..turns {
        monkes.iter().enumerate().for_each(|(monke_id, monke)| {
            while !stacks[monke_id].is_empty() {
                if let Some(item) = stacks[monke_id].pop_front() {
                    count_inspections[monke_id] += 1;

                    let mut new_item = monke.apply_operation(item);
                    if monke.throws_to_first(&mut new_item, worry) {
                        stacks[monke.monke_route[0]].push_back(new_item);
                    } else {
                        stacks[monke.monke_route[1]].push_back(new_item);
                    }
                }
            }
        });
    }
    count_inspections
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_monke_job() {
        let stacks = vec![
            VecDeque::from([79, 98]),
            VecDeque::from([54, 65, 75, 74]),
            VecDeque::from([79, 60, 97]),
            VecDeque::from([74]),
        ];

        let monkes = vec![
            Monke {
                operation: MonkeOperation::Multiply(19),
                test_div: 23,
                monke_route: [2, 3],
            },
            Monke {
                operation: MonkeOperation::Sum(6),
                test_div: 19,
                monke_route: [2, 0],
            },
            Monke {
                operation: MonkeOperation::Power,
                test_div: 13,
                monke_route: [1, 3],
            },
            Monke {
                operation: MonkeOperation::Sum(3),
                test_div: 17,
                monke_route: [0, 1],
            },
        ];
        let mut touched_items =
            turn_execute(stacks.clone(), &monkes, 20, &StressRelief::Classic(3));
        assert_eq!(101, touched_items[0]);
        assert_eq!(105, touched_items[3]);

        touched_items.sort_by(|a, b| b.cmp(a));

        assert_eq!(10605, touched_items[0] * touched_items[1]);

        let mut touched_items = turn_execute(
            stacks,
            &monkes,
            10000,
            &StressRelief::Divisor(23 * 19 * 13 * 17),
        );
        assert_eq!(52166, touched_items[0]);
        assert_eq!(52013, touched_items[3]);

        touched_items.sort_by(|a, b| b.cmp(a));

        assert_eq!(2713310158, touched_items[0] * touched_items[1]);
    }
}
