use std::io::{BufRead, stdin};

fn main() {
    println!("Hello, world!");
    
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    println!("{input}");
}

