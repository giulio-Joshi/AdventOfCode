use std::collections::HashMap;

pub fn execute(raw_input: &str) {
    let disk = get_disk_content(raw_input);

    let sum = sum_by_size_limit(&disk, &100000);

    println!("Sum with 100000 limit= {} ", sum);
    println!();

    let at_least_delete = free_to_reach(&disk, &30000000);
    println!(
        "Directory at least {} size must be deleted",
        at_least_delete
    );
    println!();
}

const TOTAL_FILESYSTEM_SIZE: usize = 70000000;

fn get_disk_content(input: &str) -> HashMap<String, usize> {
    let mut disk_content = HashMap::new();
    let mut current_path = String::from("/");

    for l in input.lines() {
        if l[0..4].eq("$ cd") {
            if l[4..6].eq(" /") {
                current_path = String::from("/");
            } else if l.len() > 6 && l[4..7].eq(" ..") {
                up_one_path(&mut current_path);
            } else {
                if current_path.len() != 1 {
                    current_path.push('/');
                }
                current_path.push_str(&l[5..]);
            }
        } else if !l[0..4].eq("$ ls") {
            let parts = l.split(' ').collect::<Vec<&str>>();
            if !parts[0].eq("dir") {
                let mut up_to_root = current_path.clone();

                loop {
                    *disk_content.entry(up_to_root.clone()).or_insert(0) +=
                        parts[0].parse::<usize>().unwrap();

                    if up_to_root.eq("/") {
                        break;
                    }
                    up_one_path(&mut up_to_root);
                }
            }
        }
    }

    disk_content
}

fn up_one_path(path: &mut String) {
    if let Some(found_last) = path.rfind('/') {
        if found_last > 0 {
            *path = path[..found_last].to_owned();
        } else {
            *path = String::from("/");
        }
    }
}

fn sum_by_size_limit(stuff: &HashMap<String, usize>, limit: &usize) -> usize {
    stuff.iter().map(|d| d.1).filter(|p| *p <= limit).sum()
}

fn free_to_reach(stuff: &HashMap<String, usize>, limit: &usize) -> usize {
    let occupied: usize = stuff.iter().map(|d| *d.1).max().unwrap();

    let minimum_to_free = limit - (TOTAL_FILESYSTEM_SIZE - occupied);

    stuff
        .iter()
        .map(|d| *d.1)
        .filter(|p| *p >= minimum_to_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {

    use crate::day7::{free_to_reach, get_disk_content, sum_by_size_limit, up_one_path};

    #[test]
    fn test_up_path() {
        let mut example_path = String::from("/test/one/two");
        up_one_path(&mut example_path);
        assert_eq!(&"/test/one", &example_path);
        up_one_path(&mut example_path);
        assert_eq!(&"/test", &example_path);
    }

    #[test]
    fn parse_test() {
        let disk = get_disk_content(data());
        assert_eq!(95437, sum_by_size_limit(&disk, &100000));
    }

    #[test]
    fn get_free_dir_size() {
        let disk = get_disk_content(data());
        assert_eq!(24933642, free_to_reach(&disk, &30000000));
    }

    fn data() -> &'static str {
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
    }
}
