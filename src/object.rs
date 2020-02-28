use std::fmt;

#[derive(Debug, Clone)]
pub enum ObjectType {
    Integer,
    Bool,
    Null,
}

/// オブジェクト
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
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
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value)     => write!(f, "{}", value),
            Object::Bool(value)        => write!(f, "{}", value),
            Object::Null               => write!(f, "null"),
            Object::ReturnValue(value) => write!(f, "{}", value),
        }
    }
}
