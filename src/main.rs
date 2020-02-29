use std::{env, fs};
use monkey_interpreter::{repl, evaluate};

fn main() {
    eprintln!("This is the Monky programming language!");
    
    match env::args().nth(1) {
        Some(fp) => {
            let input = fs::read_to_string(fp).unwrap();
            evaluate(&input).unwrap();
        },
        None => repl::start().unwrap(),
    }
}
