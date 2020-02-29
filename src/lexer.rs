use crate::token::Token;

/// 字句解析器
/// 入力ソースコードのライフタイムをライフタイムとする.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

impl<'a> Lexer<'a> {
    /// 入力ソースコードを受け取り Lexer インスタンスを生成する.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer { 
            input: input.chars(),
            cur:  '\u{0}',
            peek: '\u{0}',
        };
        lexer.read_char();
        lexer.read_char();
        lexer
    }

    /// 1 文字進む.
    fn read_char(&mut self) -> char {
        let c = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        c
    }

    /// 空白文字をスキップする.
    fn skip_whitespace(&mut self) {
        while self.cur == ' ' || self.cur == '\t' || self.cur == '\n' || self.cur == '\r' {
            self.read_char();
        }
    }

    /// 識別子一つ分を読み込みトークンに変換する.
    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();
        while is_letter(self.cur) {
            ident.push(self.read_char());
        }
        // その識別子がキーワードかどうかを判定する
        if let Some(token) = Token::keyword(&ident) {
            token
        } else {
            Token::Ident(ident)
        }
    }
    
    /// 数字一つ分を読み込みトークンに変換する.
    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while self.cur.is_ascii_digit() {
            number.push(self.read_char());
        }
        Token::Integer(number.parse().unwrap())
    }

    /// 文字列を読み込みトークンに変換する.
    fn read_string(&mut self) -> Token {
        self.read_char();

        let mut s = String::new();
        while self.cur != '"' && self.cur != '\u{0}' {
            s.push(self.read_char());
        }
        Token::String(s.to_owned())
    }

    /// 次のトークンを生成する.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.cur {
            '=' => {
                if self.peek == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            '!' => {
                if self.peek == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            },
            ';' => Token::Semicolon,
            ',' => Token::Comma, 
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus, 
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT, 
            '>' => Token::GT, 
            '"' => self.read_string(),
            '\u{0}' => Token::EOF,
            c => { 
                if is_letter(c) {
                    return self.read_ident();
                } else if c.is_ascii_digit() {
                    return self.read_number();
                } else {
                    Token::Illegal
                }
            },
        };
        self.read_char();
        token
    }
}

fn is_letter(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}


#[cfg(test)]
mod tests {
    use crate::{token::Token, lexer::Lexer};

    #[test]
    fn read() {
        let input = "Lorem ipsum\ndolor\rsit amet\t12345";
        let mut lexer = Lexer::new(input);
        assert_eq!( lexer.read_ident(), Token::Ident("Lorem".to_owned()));
        lexer.skip_whitespace();
        assert_eq!( lexer.read_ident(), Token::Ident("ipsum".to_owned()));
        lexer.skip_whitespace();
        assert_eq!( lexer.read_ident(), Token::Ident("dolor".to_owned()));
        lexer.skip_whitespace();
        assert_eq!( lexer.read_ident(), Token::Ident("sit".to_owned()));
        lexer.skip_whitespace();
        assert_eq!( lexer.read_ident(), Token::Ident("amet".to_owned()));
        lexer.skip_whitespace();
        assert_eq!( lexer.read_number(), Token::Integer(12345));
    }

    #[test]
    fn text_next_token() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
"foo bar";
10 == 10;
10 != 9;"#;
        let answers = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let, 
            Token::Ident("ten".to_owned()),
            Token::Assign, 
            Token::Integer(10),
            Token::Semicolon,
            Token::Let, 
            Token::Ident("add".to_owned()),
            Token::Assign, 
            Token::Function, 
            Token::LParen, 
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_owned()),
            Token::Plus, 
            Token::Ident("y".to_owned()),
            Token::RBrace,
            Token::Semicolon,
            Token::Let, 
            Token::Ident("result".to_owned()),
            Token::Assign, 
            Token::Ident("add".to_owned()),
            Token::LParen, 
            Token::Ident("five".to_owned()),
            Token::Comma, 
            Token::Ident("ten".to_owned()),
            Token::RParen,
            Token::Semicolon, 
            Token::Bang, 
            Token::Minus,
            Token::Slash,
            Token::Asterisk, 
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::LT, 
            Token::Integer(10),
            Token::GT, 
            Token::Integer(5),
            Token::Semicolon, 
            Token::If, 
            Token::LParen, 
            Token::Integer(5),
            Token::LT, 
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True, 
            Token::Semicolon,
            Token::RBrace, 
            Token::Else, 
            Token::LBrace, 
            Token::Return,
            Token::False, 
            Token::Semicolon,
            Token::RBrace, 
            Token::String("foo bar".to_owned()),
            Token::Semicolon,
            Token::Integer(10),
            Token::Eq, 
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEq, 
            Token::Integer(9),
            Token::Semicolon,
            Token::EOF, 
        ];

        let mut lexer = Lexer::new(input);
        for answer in answers.iter() {
            assert_eq!(&lexer.next_token(), answer);
        }
    }

    #[test]
    fn test_is_letter() {
        use super::is_letter;
        assert!( is_letter('a') );
        assert!( is_letter('f') );
        assert!( is_letter('z') );
        assert!( is_letter('A') );
        assert!( is_letter('O') );
        assert!( is_letter('Z') );
        assert!( is_letter('_') );
        assert!( !is_letter('0') );
        assert!( !is_letter(' ') );
        assert!( !is_letter('あ') );
        assert!( !is_letter('ä') );
        assert!( !is_letter('+') );
    }
}
