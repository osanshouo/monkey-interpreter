pub mod token;
pub mod lexer;
pub mod ast;
pub mod operator;
pub mod parser;
pub mod object;
pub mod env;
pub mod eval;
pub mod repl;
pub mod error;


pub fn evaluate(input: &str) -> Result<crate::object::Object, crate::error::MonkeyError> {
    let mut env = eval::Evaluator::new();
    let l = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program()?;

    env.eval(&program)
}
