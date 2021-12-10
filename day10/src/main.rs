use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;

const ROUND : ( char, char, i64) = ( '(',')' ,3);
const SQUARE : ( char, char, i64) = ( '[',']' ,57);
const GRAPH : ( char, char, i64) = ( '{','}' ,1197);
const ANGLE : ( char, char, i64) = ( '<','>' ,25137);


fn main() {
    
    let file_content = fs::read_to_string("day10/input.txt").expect("Can't find input");
    
    println!("starting data size: {}", file_content.len());

    let result = file_content.lines()
        .map(progress_count).collect::<Vec<i64>>();

        println!("Sum: {}", result.into_iter().sum::<i64>());
}

fn is_opening( token : &char ) -> bool {
    token.eq(&ROUND.0) || token.eq(&SQUARE.0) || token.eq(&GRAPH.0) || token.eq(&ANGLE.0)
}

fn correspondant( token : &char) -> &char {
    
    if token.eq(&ROUND.0) { 
        return &ROUND.1;
    }
    if token.eq(&SQUARE.0) {
        return &SQUARE.1;
    }
    if token.eq(&GRAPH.0) {
        return &GRAPH.1;
    } 
    if token.eq(&ANGLE.0) {
        return &ANGLE.1;
    }

    if token.eq(&ROUND.1) { 
        return &ROUND.0;
    }
    if token.eq(&SQUARE.1) {
        return &SQUARE.0;
    }
    if token.eq(&GRAPH.1) {
        return &GRAPH.0;
    } 
    if token.eq(&ANGLE.1) {
        return &ANGLE.0;
    }

    panic!("aaah");
}

fn error_value( token : &char) -> i64 {
    
    if token.eq(&ROUND.0) || token.eq(&ROUND.1)  { 
        return ROUND.2;
    }
    if token.eq(&SQUARE.0) || token.eq(&SQUARE.1)  {
        return SQUARE.2;
    }
    if token.eq(&GRAPH.0) || token.eq(&GRAPH.1)  {
        return GRAPH.2;
    } 
    if token.eq(&ANGLE.0) || token.eq(&ANGLE.1)  {
        return ANGLE.2;
    }

    0
}


fn progress_count( input : &str) -> i64 {

    let mut tags : Vec<char> = vec!();

    let parsed : Vec<i64> = input.chars()
        .into_iter()
        .map( |tag|  {
            if is_opening(&tag) {
                tags.push(tag);
                None

            } else {
                let check_corr = correspondant(&tag);

                if let Some( previous )  = tags.pop() {
                    if previous.eq(check_corr) {
                         return None;
                    }
                           
                } 
                Some( error_value( &tag))
            }
        }).flatten()
        .collect();


        if parsed.len() > 0 {
            parsed.into_iter().product()
        } else {
            0_i64
        }
}

#[cfg(test)]
mod test {
    use crate::progress_count;

    #[test]
    fn parse(){

        let rows =  data() ;

        let result = rows.lines()
            .map(progress_count).collect::<Vec<i64>>();

        assert_eq!(26397 as i64, result.into_iter().sum::<i64>());

    }

    fn data() -> &'static str {
        r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#
    }
}