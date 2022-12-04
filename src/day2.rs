use std::{str::FromStr, collections::HashMap};


pub fn execute( raw_input: &str) {

    let passwords = parse_passwords(raw_input);

    let valid_ones = passwords.iter().filter(is_valid_policy).count();
    println!("Found {valid_ones}  valid passwords");

    let valid_ones = passwords.iter().filter(is_valid_toboggan).count();
    println!("Found {valid_ones}  valid toboggan style");
}


fn is_valid_toboggan<'r>( data: &'r &(Policy, String)) -> bool {
    data.0.check_toboggan(&data.1)
}

fn is_valid_policy<'r>( data: &'r &(Policy, String)) -> bool {
    data.0.check_password(&data.1)
}

fn count_characters(password: &str) -> HashMap<String, usize> {
    let mut all_chars : HashMap<String, usize> = HashMap::new();

    for char in password.chars() {
        let find = char.to_string();
        let entry = all_chars.entry(find)
            .or_insert(0);
        *entry+=1;
    }

    all_chars
}

struct Policy {
    pub min: usize,
    pub max: usize,
    pub letter: char
}

impl Policy {
    fn check_password(&self, password: &str) -> bool {
        let found_chars = count_characters(password);

        if let Some(found) = found_chars.get_key_value(&self.letter.to_string()) {

            return *found.1 >= self.min && *found.1 <= self.max;
        }

        false
    }

    fn check_toboggan(&self, password: &str) -> bool {

        let char_one = password.chars().nth(self.min-1).unwrap();
        let char_two = password.chars().nth(self.max-1).unwrap();
        (self.letter == char_one) ^ (self.letter == char_two)
    }
}

impl FromStr for Policy {
    type Err=String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let bounds :Vec<usize>=parts[0].split('-').map(|x| x.parse::<usize>().unwrap()).collect();
        Ok( Self {  min: bounds[0], max: bounds[1] , letter: parts[1].chars().next().unwrap()} )
    }
}

fn parse_passwords( input: &str ) -> Vec<(Policy, String)> {
    input.lines().map(|r| {
        let parts : Vec<&str> = r.split(':').collect();
        (parts[0].parse::<Policy>().unwrap(), parts[1].trim().to_owned())
    }).collect()
}

#[cfg(test)]
mod test {
    use super::parse_passwords;

    #[test]
    fn valid_password( ) {
        let parsed =  parse_passwords(data());
        assert!(parsed[0].0.check_password(&parsed[0].1));
    }


    #[test]
    fn valid_toboggan( ) {
        let parsed =  parse_passwords(data());
        assert!(parsed[0].0.check_toboggan(&parsed[0].1));
        assert!(!parsed[1].0.check_toboggan(&parsed[1].1));
    }


    #[test]
    fn test_parsing( ){
        let parsed = parse_passwords(data());

        assert_eq!(3, parsed.len());
        assert_eq!(1, parsed[0].0.min);
        assert_eq!(3, parsed[0].0.max);
        assert_eq!('a', parsed[0].0.letter);

        assert_eq!("abcde", parsed[0].1);
    }


    fn data() -> &'static str {
        r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#
    }
}