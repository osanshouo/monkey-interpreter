use crate::{
    token::Token,
    operator,
    object::ObjectType,
};

#[derive(Debug, Clone)]
pub enum MonkeyError {
    NotFoundPrefixTreatment,
    InvalidToken(Token),
    UnexpectedToken{expected: Token, got: Token},
    TypeMismatch(ObjectType, operator::Infix, ObjectType),
    UnknownOperator(ObjectType, operator::Infix, ObjectType),
    IdentifierNotFound(String),
    IncorrectNumberOfArgs{expected: usize, got: usize},
}
