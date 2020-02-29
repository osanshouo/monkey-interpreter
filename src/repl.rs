use std::io::{self, BufRead};
use crate::{
    lexer::Lexer,
    parser::Parser,
    eval::Evaluator,
};

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), io::Error> {
    let mut env = Evaluator::new();

    eprint!("{}", PROMPT);
    for line in io::stdin().lock().lines() {
        let input = line?;

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(e) => {
                eprint!("[ERROR] {:?}\n{}", e, PROMPT);
                continue;
            },
        };

        match env.eval(&ast) {
            Ok(obj) => println!("{}", obj),
            Err(e) => {
                eprint!("[ERROR] {:?}\n{}", e, PROMPT);
                continue;
            },
        }
        eprint!("{}", PROMPT);
    }
    eprintln!("");
    Ok(())
}
