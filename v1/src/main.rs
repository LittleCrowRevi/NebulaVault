use std::io::{BufRead, stdin, stdout};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

fn main() {
    println!("Hello, world!");
    
    let run = true;
    while run {
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Printer::info(input);
        
    }
}

struct Printer;

impl Printer {
    
    pub fn info(message: String) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            Print(message),
            ResetColor
        ).unwrap()
    }
}