use std::fmt;

/// 前置演算子
#[derive(Debug, Clone, Copy)]
pub enum Prefix {
    Bang,
    Minus,
}
impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Bang => write!(f, "!"),
            Prefix::Minus => write!(f, "-"),
        }
    }
}

/// 中置演算子
#[derive(Debug, Clone, Copy)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    LT,
    GT,
}
impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Infix::Plus     => write!(f, "+"),
            Infix::Minus    => write!(f, "-"),
            Infix::Asterisk => write!(f, "*"),
            Infix::Slash    => write!(f, "/"),
            Infix::Eq       => write!(f, "=="),
            Infix::NotEq    => write!(f, "!="),
            Infix::LT       => write!(f, "<"),
            Infix::GT       => write!(f, ">"),
        }
    }
}

/// 中置演算子の優先順位
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[cfg(test)]
mod tests {
    use crate::operator;

    #[test]
    fn test_operator_precedence() {
        assert_eq!( operator::Precedence::Product, operator::Precedence::Product );
        assert!( operator::Precedence::Lowest < operator::Precedence::Equals );
        assert!( operator::Precedence::Call >= operator::Precedence::Prefix );
    }
}