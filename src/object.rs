use std::fmt;
use crate::{ast, env::Environment};

#[derive(Debug, Clone)]
pub enum ObjectType {
    Integer,
    Bool,
    Null,
}

/// オブジェクト
#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i32),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
    Function{parameters: Vec<ast::Expression>, body: ast::Statement, env: Environment},
}
impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Bool(value) => *value,
            _ => true,
        }
    }
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(x), Object::String(y)) => x == y,
            (Object::Integer(x), Object::Integer(y)) => x == y,
            (Object::Bool(x), Object::Bool(y)) => x == y,
            (Object::Null, Object::Null) => true,
            // (Object::Function{..}, Object::Function{..}) => {
            //     format!("{}", self) == format!("{}", other)
            // },
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s)          => write!(f, "{}", s),
            Object::Integer(value)     => write!(f, "{}", value),
            Object::Bool(value)        => write!(f, "{}", value),
            Object::Null               => write!(f, "null"),
            Object::ReturnValue(value) => write!(f, "{}", value),
            Object::Function{parameters, body, ..} => {
                write!(f, "fn({}){{{}}}", 
                    parameters.iter().map(|expr| format!("{}", expr)).collect::<Vec<_>>().join(","), 
                    body
                )
            },
        }
    }
}
