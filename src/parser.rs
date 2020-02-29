use crate::{
    token::Token,
    lexer::Lexer,
    ast,
    operator::self,
    error::MonkeyError,
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}
impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser { 
            l, 
            cur_token: Token::Illegal, 
            peek_token: Token::Illegal,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn cur_token_is(&self, expected_token: Token) -> bool {
        self.cur_token == expected_token
    }

    fn peek_token_is(&self, expected_token: Token) -> bool {
        self.peek_token == expected_token
    }

    pub(crate) fn expect_peek(&mut self, expected_token: Token) -> Result<(), MonkeyError> {
        if self.peek_token_is(expected_token.clone()) {
            self.next_token();
            Ok(())
        } else {
            Err(MonkeyError::UnexpectedToken {
                expected: expected_token,
                got: self.peek_token.clone()
            })
        }
    }

    fn cur_precedence(&self) -> operator::Precedence {
        self.cur_token.precedence()
    }
    
    fn peek_precedence(&self) -> operator::Precedence {
        self.peek_token.precedence()
    }
    
    pub fn parse_program(&mut self) -> Result<ast::Program, MonkeyError> {
        let mut program = ast::Program::new();

        while !self.cur_token_is(Token::EOF) {
            program.statements.push(
                self.parse_statement()?
            );
            self.next_token();
        }

        Ok(program)
    }

    /// 文をパースする
    /// let 文, return 文, 式文のいずれかを判断し適切なメソッドを呼び出す
    fn parse_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        match self.cur_token {
            Token::Let    => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _                 => self.parse_expression_statement(),
        }
    }

    /// let 文をパース
    fn parse_let_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        // let を飛ばして次に行く
        self.next_token();

        // let の次には識別子が来るはずなので, それを ast::Expression に変換する.
        let ident = if let Token::Ident(ident) = &self.cur_token {
            ast::Expression::Ident(ident.to_owned())
        } else { 
            return Err(MonkeyError::UnexpectedToken{
                expected: Token::Ident("".to_owned()), got: self.cur_token.clone()
            });
        };

        // 識別子の次には等号が来るはず. なければエラーを返し終了.
        self.expect_peek(Token::Assign)?;

        // 等号を飛ばして次に行く
        self.next_token();

        // 右辺の値を取得
        let value = self.parse_expression(operator::Precedence::Lowest)?;

        // 次がセミコロンなら読み飛ばす. 
        if !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        
        Ok(ast::Statement::Let{ident, value})
    }

    /// return 文をパース
    fn parse_return_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        self.next_token();

        // 戻り値を取得
        let value = self.parse_expression(operator::Precedence::Lowest)?;

        self.expect_peek(Token::Semicolon)?;

        Ok(ast::Statement::Return(value))
    }

    /// 式文をパース
    fn parse_expression_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        let expr = self.parse_expression(operator::Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Expression(expr))
    }

    /// lexer をパースして ast::Expression を生成する.
    /// 呼び出し時のカーソル位置が読み出す expr の先頭で, このメソッド終了時には expr の最後のトークンにいる.
    fn parse_expression(&mut self, precedence: operator::Precedence) -> Result<ast::Expression, MonkeyError> {
        // 単独の式または前置演算子
        let mut left = match &self.cur_token {
            Token::Ident(ident)   => ast::Expression::Ident(ident.to_owned()),
            Token::String(s)      => ast::Expression::String(s.to_owned()),
            Token::Integer(value) => ast::Expression::Integer(*value),
            Token::True       => ast::Expression::Bool(true),
            Token::False      => ast::Expression::Bool(false),
            Token::Bang       => self.parse_prefix_expression()?,
            Token::Minus      => self.parse_prefix_expression()?,
            Token::LParen     => self.parse_grouped_expression()?,
            Token::If         => self.parse_if_expression()?,
            Token::Function   => self.parse_function_literal()?,
            token             => { return Err(MonkeyError::InvalidToken(token.clone())); },
        };
        
        // 次に中置演算子が来る場合はここで処理する
        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::Plus     => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::Minus    => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::Slash    => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::Asterisk => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::Eq       => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::NotEq    => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::LT       => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::GT       => { self.next_token(); left = self.parse_infix_expression(left)?; },
                Token::LParen   => { self.next_token(); left = self.parse_call_expression(left)?; },
                _               => { return Ok(left); },
            }
        }
        
        Ok(left)
    }
    
    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        self.next_token();

        let exp = self.parse_expression(operator::Precedence::Lowest)?;
        self.expect_peek(Token::RParen)?;

        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        self.expect_peek(Token::LParen)?;
        let condition = Box::new(
            self.parse_expression(operator::Precedence::Lowest)?
        );
        if !self.cur_token_is(Token::RParen) {
            return Err(MonkeyError::UnexpectedToken{expected: Token::RParen, got: self.cur_token.clone()});
        };

        self.expect_peek(Token::LBrace)?;
        let consequence = self.parse_block_statement()?;
        let consequence = Box::new(consequence);

        let alternative = if self.peek_token_is(Token::Else) {
            self.next_token();
            self.expect_peek(Token::LBrace)?;
            let alt = self.parse_block_statement()?;
            Some(Box::new(alt))
        } else { None };

        Ok(ast::Expression::If{condition, consequence, alternative})
    }

    fn parse_function_literal(&mut self) -> Result<ast::Expression, MonkeyError> {
        // Token::Function に続いて Token::LParen が来るはず
        self.expect_peek(Token::LParen)?;
        let parameters = self.parse_function_parameters()?;

        // 引数リストが終わったら Token::LBrace に続いて関数の本体が来るかず
        self.expect_peek(Token::LBrace)?;
        let body = self.parse_block_statement()?;

        Ok(ast::Expression::Function{parameters, body: Box::new(body)})
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<ast::Expression>, MonkeyError> {
        let mut idents = Vec::new();

        if self.peek_token_is(Token::RParen) {
            // 引数がない場合, Token::RParen を飛ばして終了
            self.next_token();
            Ok(idents)
        } else {
            // 引数が 1 つ以上ある場合
            self.next_token();

            while {
                if let Token::Ident(ident) = &self.cur_token {
                    idents.push( ast::Expression::Ident(ident.to_owned()) );
                } else { return Err(MonkeyError::InvalidToken(self.cur_token.clone())); }

                self.peek_token_is(Token::Comma)
            } {
                self.next_token();
                self.next_token();
            }

            self.expect_peek(Token::RParen)?;

            Ok(idents)
        }
    }

    fn parse_call_expression(&mut self, function: ast::Expression) -> Result<ast::Expression, MonkeyError> {
        let mut arguments = Vec::new();

        if self.peek_token_is(Token::RParen) {
            self.next_token();
        } else {
            self.next_token();
            while {
                arguments.push(self.parse_expression(operator::Precedence::Lowest)?);
                self.peek_token_is(Token::Comma)
            } {
                self.next_token();
                self.next_token();
            }

            self.expect_peek(Token::RParen)?;
        }

        Ok(ast::Expression::Call{function: Box::new(function), arguments})
    }

    fn parse_block_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        self.next_token();

        let mut blocks = Vec::new();
        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::EOF) {
            blocks.push(
                self.parse_statement()?
            );
            self.next_token();
        }

        Ok(ast::Statement::Block(blocks))
    }

    fn parse_prefix_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        let op = match self.cur_token {
            Token::Bang  => operator::Prefix::Bang,
            Token::Minus => operator::Prefix::Minus,
            _            => { return Err(MonkeyError::InvalidToken(self.cur_token.clone())); },
        };

        self.next_token();

        let right = self.parse_expression(operator::Precedence::Prefix)?;

        Ok(ast::Expression::Prefix{op, right: Box::new(right)})
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Result<ast::Expression, MonkeyError> {
        let op = match self.cur_token {
            Token::Plus     => operator::Infix::Plus,
            Token::Minus    => operator::Infix::Minus,
            Token::Asterisk => operator::Infix::Asterisk,
            Token::Slash    => operator::Infix::Slash,
            Token::Eq       => operator::Infix::Eq,
            Token::NotEq    => operator::Infix::NotEq,
            Token::LT       => operator::Infix::LT,
            Token::GT       => operator::Infix::GT,
            _               => { return Err(MonkeyError::InvalidToken(self.cur_token.clone())); },
        };

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::Infix{op, left: Box::new(left), right: Box::new(right)})
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast;

    
    #[test]
    fn parser_let_test1() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;"#;

        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got {}",
                program.statements.len());
        }
        let expected_identifier = vec![
            "x",
            "y",
            "foobar",
        ];

        for (i, ident) in expected_identifier.iter().enumerate() {
            test_let_statement(&program.statements[i], ident);
        }
    }

    fn test_let_statement(stmt: &ast::Statement, expected_name: &str) {
        if let ast::Statement::Let{ident,..} = stmt {
            if let ast::Expression::Ident(ident) = ident {
                assert_eq!( ident, expected_name );
            }
        } else {
            panic!("expected ast::Statement::Let, but got {:?}", stmt);
        }
    }

    #[test]
    fn parser_return_test() {
        let input = r#"
return 5;
return 10;
return 993322;"#;

        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got {}",
                program.statements.len());
        }

        for stmt in program.statements.iter() {
            if let ast::Statement::Return(_) = stmt {
            } else {
                panic!("expected Return, but got {:?}", &stmt);
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("Failed to parse input!");

        if program.statements.len() != 1 {
            panic!("program should have one statement, but has {}", 
                program.statements.len());
        }

        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::Ident(ident) = expr {
                assert_eq!(ident, "foobar");
            } else { panic!("Incorrect expression"); }
        } else {
            panic!("Incorrect statement");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("Failed to parse input!");

        if program.statements.len() != 1 {
            panic!("program should have one statement, but has {}", 
                program.statements.len());
        }

        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::Integer(value) = expr {
                assert_eq!(value, &5);
            } else { panic!("Incorrect expression"); }
        } else {
            panic!("Incorrect statement");
        }
    }

    #[test]
    fn operator_precedence() {
        let inputs = [ "5 + 5 * 10;", "- a * b", ];
        let answers = [ "(5+(5*10))", "((-a)*b)", ];

        for (input, answer) in inputs.iter().zip(answers.iter()) {
            let l = Lexer::new(&input);
            let mut p = Parser::new(l);
            let program = p.parse_program().expect("Failed to parse input!");

            if program.statements.len() != 1 {
                panic!("program should have one statement, but has {}", 
                    program.statements.len());
            }
    
            let stmt = &program.statements[0];
            if let ast::Statement::Expression(expr) = stmt {
                assert_eq!(&format!("{}", &expr), answer);
            } else {
                panic!("Incorrect statement");
            }
        }
    }

    #[test]
    fn parse_expression() {
        // 入力 Monky コードと期待される AST (の ASCII 表現) の組
        let problem = [
            ("-5;", "(-5);"),
            ("5 + 5 * 10;", "(5+(5*10));"),
            ("(5 + 5 ) * 10;", "((5+5)*10);"),
            ("true;", "true;"),
            ("let x = false;", "let x = false;"),
            ("if (x) { 2 + 3 }", "if(x){(2+3);};"),
            ("if (x) { 2 + 3; 4/2; } else { 5; }", "if(x){(2+3);(4/2);}else{5;};"),
            ("fn(){ 5; }", "fn(){5;};"),
            ("fn(x, y) { x + y; }", "fn(x,y){(x+y);};"),
            ("add(1, 2*3);", "add(1,(2*3));"),
            ("add(1, minus(4, -1));", "add(1,minus(4,(-1)));"),
        ];

        for (input, answer) in problem.iter() {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().expect("Failed to parse");

            if program.statements.len() != 1 {
                panic!("The number of statements is {}, not 1", program.statements.len());
            }

            let stmt = format!("{}", &program.statements[0]);
            eprint!("{}", &stmt);
            assert_eq!(&stmt, answer);
            eprintln!(" ... ok!");
        }
    }
}