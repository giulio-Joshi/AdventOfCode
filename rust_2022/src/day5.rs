use std::str::FromStr;

pub fn execute(raw_input: &str) {
    let parts = raw_input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = read_stacks(parts[0]);
    let instructions: Vec<Instruction> = parts[1].lines().flat_map(Instruction::from_str).collect();

    (0..instructions.len()).for_each(|i| {
        instructions[i].perform(&mut stacks);
    });

    println!("Value with single moving part: ");

    (0..stacks.len()).for_each(|i| {
        print!("{}", stacks[i].pop().unwrap_or(' '));
    });

    // I need to create a new stack from the original input
    let mut stacks = read_stacks(parts[0]);
    (0..instructions.len()).for_each(|i| {
        instructions[i].crate9000(&mut stacks);
    });

    println!();
    println!("Value with stack moving parts: ");

    (0..stacks.len()).for_each(|i| {
        print!("{}", stacks[i].pop().unwrap_or(' '));
    });
}

fn read_stacks(input: &str) -> Vec<Vec<char>> {
    let mut rows = input.lines().rev();
    let num_stacks = rows.next().unwrap().len() / 4 + 1;

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);

    rows.for_each(|line| {
        for i in 0..num_stacks {
            if stacks.len() < i + 1 {
                stacks.push(vec![]);
            }
            if let Some(letter) = line.chars().nth(1 + (i * 4)) {
                if letter != ' ' {
                    stacks[i].push(letter);
                }
            }
        }
    });
    stacks
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let pieces = input_data.split(' ').collect::<Vec<&str>>();

        let amount = usize::from_str(pieces[1]).unwrap();
        let from = usize::from_str(pieces[3]).unwrap();
        let to = usize::from_str(pieces[5]).unwrap();

        Ok(Instruction { amount, from, to })
    }
}

impl Instruction {
    fn perform(&self, stack: &mut [Vec<char>]) {
        (0..self.amount).for_each(|_i| {
            let element = stack[self.from - 1].pop().unwrap();
            stack[self.to - 1].push(element);
        });
    }

    fn crate9000(&self, stack: &mut [Vec<char>]) {
        let how_much = (stack[self.from - 1].len() - self.amount)..stack[self.from - 1].len();
        let mut removed = stack[self.from - 1].drain(how_much).collect::<Vec<char>>();
        stack[self.to - 1].append(&mut removed);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_test() {
        let parts = data().split("\n\n").collect::<Vec<&str>>();
        let mut stacks = read_stacks(parts[0]);

        let instructions: Vec<Instruction> =
            parts[1].lines().flat_map(Instruction::from_str).collect();

        assert_eq!(4, stacks.len());
        assert_eq!(4, instructions.len());

        (0..instructions.len()).for_each(|i| {
            instructions[i].perform(&mut stacks);
        });

        assert_eq!(vec!['C'], stacks[0]);
        assert_eq!(vec!['M'], stacks[1]);
        assert_eq!(vec!['P', 'D', 'N', 'Z'], stacks[2]);
    }

    #[test]
    fn parse_second() {
        let parts = data().split("\n\n").collect::<Vec<&str>>();
        let mut stacks = read_stacks(parts[0]);

        let instructions: Vec<Instruction> =
            parts[1].lines().flat_map(Instruction::from_str).collect();

        (0..instructions.len()).for_each(|i| {
            instructions[i].crate9000(&mut stacks);
        });

        assert_eq!(vec!['M'], stacks[0]);
        assert_eq!(vec!['C'], stacks[1]);
        assert_eq!(vec!['P', 'Z', 'N', 'D'], stacks[2]);
    }

    fn data() -> &'static str {
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
    }
}
