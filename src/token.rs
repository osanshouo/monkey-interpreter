use crate::operator::Precedence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EOF,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Ident(String),
    Integer(i32),
    String(String),
}

impl Token {
    /// 与えられた識別子がキーワードに一致したらそのキーワードに対応するトークンを返す.
    /// キーワードに一致しなければ `None` を返す.
    pub fn keyword(ident: &str) -> Option<Token> {
        match ident {
            "fn"     => Some(Token::Function),
            "let"    => Some(Token::Let     ),
            "true"   => Some(Token::True    ),
            "false"  => Some(Token::False   ),
            "if"     => Some(Token::If      ),
            "else"   => Some(Token::Else    ),
            "return" => Some(Token::Return  ),
            _        => None,
        }
    }

    /// 中置演算子の優先順位を定義する. crate::operator::Precedence 参照.
    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Eq       => Precedence::Equals,
            Token::NotEq    => Precedence::Equals,
            Token::LT       => Precedence::LessGreater,
            Token::GT       => Precedence::LessGreater,
            Token::Plus     => Precedence::Sum,
            Token::Minus    => Precedence::Sum,
            Token::Slash    => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::LParen   => Precedence::Call,
            _                => Precedence::Lowest,
        }
    }
}
