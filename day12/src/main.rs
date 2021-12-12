use std::{ops::Add, collections::HashSet};

fn main() {
    println!("Hello, world!");
}



pub fn find_paths( map: &[(String,String)] ) -> i32 {


    let mut all_paths : HashSet<String>= HashSet::new();
    all_paths.insert("start".into());
    
    let mut path_is_open = true;

    while path_is_open {

        let new_paths = map.into_iter()
            .fold(HashSet::new(), | mut new_paths : HashSet<String> ,step| {

                //let mut to_add : Vec<String> = vec!();

                all_paths
                    .iter()
                    .for_each(| prev| { 

                        if prev.ends_with( "end") {
                            new_paths.insert(prev.into());
                            return;
                        }


                        if prev.ends_with( &step.1 ) && !step.0.eq("start") {

                            let dest_is_small = step.0.chars().next().unwrap().is_lowercase();

                            if  !dest_is_small || (!prev.contains(&step.0) && dest_is_small ) {
                                let path_step = [ prev, ",", &step.0 ];
                                new_paths.insert(path_step.concat());
                            }

                        } else if prev.ends_with( &step.0 ) && !step.1.eq("start") {
                            let dest_is_small = step.1.chars().next().unwrap().is_lowercase();
                            if !dest_is_small || (dest_is_small && !prev.contains(&step.1)) {
                                let path_step = [ prev, ",", &step.1 ];
                                new_paths.insert(path_step.concat());
                            }
                        } 

                    });

                    new_paths
            });

            all_paths= new_paths;


            /*println!("Calculated paths: \n");
            all_paths.iter()
                .for_each(|x| println!("{}", x));*/

        path_is_open = Option::is_some(&all_paths.iter()
            .find(|&x| !x.ends_with("end")))
        
    }


    all_paths.len() as i32
}

pub fn explore(input: &str) -> Vec<(String,String)> {

    input.lines()
        .map( |z| {
            let mut parts=z.split('-');
            (parts.next().unwrap().into(),parts.next().unwrap().into())

        })
        .collect()
}


#[cfg(test)]
mod test {

    use crate::{find_paths, explore};


    #[test]
    fn test_find_paths() {

        let paths = explore(data());

        //println!("{:?}",paths);

        assert_eq!(19, find_paths(&paths));

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