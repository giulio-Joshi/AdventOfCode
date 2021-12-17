use std::fs;

fn main() {
    let file_content = fs::read_to_string("day16/input.txt").expect("Can't find input");
    println!("Resulting versions: {}", count_versions( &to_binary(&file_content) ));

}

fn count_versions( binary : &str ) -> u32 {


    let mut versions: Vec<u32> = vec!();
    let mut cursor: usize= 0;

    while cursor < binary.len()-6 {


        let version = &binary[cursor..cursor+3];
        let id = &binary[cursor+3..cursor+6];

        println!("Found version: {}", usize::from_str_radix(&version, 2).unwrap());

        cursor+=6;

        match id {
            "100" => {
                // Literal
                
                let mut value = String::from("");
                loop {
                    
                    value.push_str( &binary[cursor+1..cursor+5]);
                    cursor+=5;
                    // check first bit to continue
                    if &binary[cursor-5..cursor-4] == "0" || cursor+5 > binary.len() {
                        if (binary.len()-cursor ) < 4 {
                            cursor=binary.len();
                        }
                        break;
                    }
                }
            }
            _ => {
                if binary.len() - cursor < 15 {
                    cursor=binary.len();
                } else {

                    if  binary[cursor..cursor+1].eq("1") {
                        let count_string = &binary[cursor+1..cursor+12];
                        let mut count=  usize::from_str_radix(&count_string, 2).unwrap();
                        cursor+=12;
                        // Length is bits length

                        //while count > 0 {

                        versions.push( count_versions( &binary[cursor..binary.len()]));
                        cursor = binary.len();
                        //cursor+=11;
                            //count-=1;
                        //}

                        // 
                    } else {
                        let size=usize::from_str_radix( &binary[cursor+1..cursor+16], 2).unwrap();
                        cursor+=16;

                        // count is subpackets counts
                        versions.push( count_versions( &binary[cursor..cursor+size]));
                        cursor+=size;
                    }
                }
                
            }
        }

        versions.push(u32::from_str_radix( version, 2 ).unwrap());
        

    }


    versions.into_iter().sum()

}

fn to_binary( input: &str) -> String {
    input.chars().map(|x| {
        match x {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("vòccòdzjø the fuck is {}", x)   
        }

    }).collect::<Vec<&str>>()
    .join("")
}


#[cfg(test)]
mod test {
    use crate::{count_versions, to_binary};



    #[test]
    fn parse_std(){
    
        println!("VAI CON {}",DATA_0);
        assert_eq!( 6, count_versions( &to_binary(DATA_0) ));
        println!("E poi...");
        assert_eq!( 14, count_versions( &to_binary("EE00D40C823060")));
        

    }

    #[test]
    fn parse_provided_01(){
        assert_eq!( 16, count_versions( &to_binary(DATA_V4) ));
    }

    #[test]
    fn parse_provided_02(){
        assert_eq!( 12, count_versions( &to_binary(DATA_V3) ));
        
        
    }

    #[test]
    fn parse_provided_03(){
        assert_eq!( 23, count_versions( &to_binary(DATA_V3_BIS) ));
    }

    #[test]
    fn parse_provided_04(){
        assert_eq!( 31, count_versions( &to_binary(DATA_OP) ));
    }


    const DATA_0 : &str = "D2FE28";
    const DATA_V4 : &str = "8A004A801A8002F478";
    const DATA_V3 : &str = "620080001611562C8802118E34";
    const DATA_V3_BIS : &str = "C0015000016115A2E0802F182340";
    const DATA_OP : &str = "A0016C880162017C3686B18A3D4780";

}