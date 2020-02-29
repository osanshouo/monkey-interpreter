use std::fmt;
use crate::operator;

/// 文 (statement) を表す enum.
#[derive(Debug, Clone)]
pub enum Statement {
    Let{ident: Expression, value: Expression},
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let{ident, value} => write!(f, "let {} = {};", ident, value),
            Statement::Return(value)     => write!(f, "return {};", value),
            Statement::Expression(value) => write!(f, "{};", value),
            Statement::Block(blocks)     => {
                for stmt in blocks.iter() { write!(f, "{}", stmt)?; }
                Ok(())
            },
        }
    }
}

/// AST のルートノード
#[derive(Debug, Clone)]
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Self { Self { statements: Vec::new() } }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in self.statements.iter() {
            write!(f, "{}\n", stmt)?;
        }
        Ok(())
    }
}

/// 式 (expression) を表す enum.
#[derive(Debug, Clone)]
pub enum Expression {
    Ident(String),
    String(String),
    Integer(i32),
    Bool(bool),
    Prefix {op: operator::Prefix, right: Box<Expression>},
    Infix  {op: operator::Infix,  left: Box<Expression>, right: Box<Expression>},
    If       {condition: Box<Expression>, consequence: Box<Statement>, alternative: Option<Box<Statement>>},
    Function {parameters: Vec<Expression>, body: Box<Statement>},
    Call     {function: Box<Expression>, arguments: Vec<Expression>},
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(value) => write!(f, "{}", &value),
            Expression::String(value) => write!(f, "{}", &value),
            Expression::Integer(value) => write!(f, "{}", value),
            Expression::Bool(value) => write!(f, "{}", value),
            Expression::Prefix{op, right} => write!(f, "({}{})", op, right),
            Expression::Infix{op, left, right} => write!(f, "({}{}{})", left, op, right),
            Expression::If{condition, consequence, alternative} => {
                match alternative {
                    Some(alt) => write!(f, "if({}){{{}}}else{{{}}}", condition, consequence, alt),
                    None => write!(f, "if({}){{{}}}", condition, consequence),
                }
            },
            Expression::Function{parameters, body} => {
                write!(f, "fn({}){{{}}}", 
                    parameters.iter().map(|expr| -> &str { match expr {
                            Expression::Ident(ident) => ident,
                            _ => unreachable!(),
                    }}).collect::<Vec<_>>().join(","),
                    body
                )
            },
            Expression::Call{function, arguments} => write!(f, "{}({})",
                function,
                arguments.iter().map(|expr| format!("{}", &expr)).collect::<Vec<_>>().join(","),
            ),
        }
    }
}
