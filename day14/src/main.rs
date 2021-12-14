use std::{fs, collections::HashMap};

fn main() {
    let file_content = fs::read_to_string("day14/input.txt").expect("Can't find input");
    let template = read_template(&file_content);
    let pairs = read_couples(&file_content);

    let result_in_10 = calc_most_less_minus( &apply_pairs(&template, &pairs, 40) );

    println!("Counted value part 2 {}", result_in_10);
}

fn calc_most_less_minus( polymer: &str ) -> u32 {

    let mut counter  : HashMap<char, u32> = HashMap::new();

    polymer.chars()
        .for_each( |x|  { 
            *counter
                .entry(x)
                .or_insert(0) += 1 ;  });


    let most_common= counter.iter().fold(0_u32, |accum, p|{
        if p.1.gt(&accum) {
            *p.1 
        } else {
            accum
        }
    });

    let less_common = counter.iter()
        .fold( most_common, |accum, p|{
            
            if p.1.lt(&accum) {
                *p.1 
            } else {
                accum
            }
        });

    most_common - less_common
}


fn apply_pairs( template: &str, pairs: &[Pair], steps: u32 ) -> String {

    if steps % 5 == 0 {
        println!("Step is {}, string is : {} long ", steps, template.len());
    }

    let mut polymer: Vec<String> = vec!();

    for i in 0..(template.len()-1) {
       let couple = &template[i..(i+2)];
       if let Some(pair) = pairs.iter()
            .find(|&p| p.matches(couple)) {

                if polymer.is_empty() {
                    polymer.push( String::from(&couple[0..1]));
                }
                polymer.push( String::from(&pair.add));
                polymer.push( String::from(&couple[1..2]));
            }
   }

   if steps-1 > 0 {
       apply_pairs( &polymer.concat(), pairs, steps -1 )
   } else {
        polymer.concat()
   }
}

struct Pair {
    couple: String,
    add: String
}

impl Pair {

    fn new( data: &str ) -> Self {
        let mut v = data.split(" -> ");
        Pair{ couple: String::from(v.next().unwrap()) ,
            add:String::from(v.next().unwrap()) }
    }

    fn matches(&self, polymer_part: &str) -> bool {
        self.couple.eq(polymer_part)
    }
}

fn read_template(i: &str) -> String {
    i.lines().take(1).map(String::from).collect::<Vec<String>>()[0].to_owned()
}

fn read_couples(input: &str) -> Vec<Pair> {
    input
        .lines()
        .skip(2)
        .map(Pair::new)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{read_couples, read_template, apply_pairs, calc_most_less_minus};

    #[test]
    fn steps() {

        let template= read_template(data());
        let pairs = read_couples(data());

        assert_eq!("NCNBCHB", apply_pairs(&template, &pairs, 1) );
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", apply_pairs(&template, &pairs, 4) );

        assert_eq!(1588, calc_most_less_minus( &apply_pairs(&template, &pairs, 10) ) );
    }

    #[test]
    fn test_parsing() {
        let template = read_template(data());
        assert_eq!("NNCB", template);

        let pairs = read_couples(data());
        assert_eq!(16, pairs.len());
    }

    fn data() -> &'static str {
        r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#
    }
}
