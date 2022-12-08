use std::collections::HashMap;

pub fn execute(raw_input: &str) {
    let marker_position = find_marker(raw_input.lines().next().unwrap(), 4).unwrap();
    println!("Found marker at {marker_position}");
    let marker_position = find_marker(raw_input.lines().next().unwrap(), 14).unwrap();
    println!("Found message-start at {marker_position}");
}

fn find_marker(input: &str, lenght: usize) -> Option<usize> {
    for cnt in 0..input.len() - lenght {
        let counts = count_characters(&input[cnt..cnt + lenght]);
        if counts.len() == lenght {
            return Some(cnt + lenght);
        }
    }

    None
}

fn count_characters(password: &str) -> HashMap<String, usize> {
    let mut all_chars: HashMap<String, usize> = HashMap::new();

    for char in password.chars() {
        let find = char.to_string();
        let entry = all_chars.entry(find).or_insert(0);
        *entry += 1;
    }

    all_chars
}

#[cfg(test)]
mod test {

    use super::find_marker;

    #[test]
    fn find_marker_test() {
        let marker = find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap();
        assert_eq!(7, marker);
        let marker = find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap();
        assert_eq!(5, marker);
        let marker = find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap();
        assert_eq!(6, marker);
        let marker = find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap();
        assert_eq!(10, marker);
        let marker = find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap();
        assert_eq!(11, marker);
    }

    #[test]
    fn find_message_start() {
        let marker = find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap();
        assert_eq!(19, marker);
        let marker = find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap();
        assert_eq!(23, marker);
        let marker = find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap();
        assert_eq!(23, marker);
        let marker = find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap();
        assert_eq!(29, marker);
        let marker = find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap();
        assert_eq!(26, marker);
    }
}
