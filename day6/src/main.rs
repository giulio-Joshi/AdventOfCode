fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {

    fn read_data() -> String {
        String::from("3,4,3,1,2")
    }
}