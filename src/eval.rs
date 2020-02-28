use std::collections::HashMap;
use crate::{
    ast,
    operator,
    object::{Object, ObjectType},
    error::MonkeyError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment { store: HashMap::new(), outer: None }
    }

    fn virtual_environment(&self) -> Environment {
        Environment { store: HashMap::new(), outer: Some(Box::new(self.clone())) }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(env) => env.get(key),
                None      => None,
            },
        }
    }

    pub fn eval(&mut self, program: &ast::Program) -> Result<Object, MonkeyError> {
        let mut result = Object::Null;

        for stmt in program.statements.iter() {
            result = self.eval_statement(stmt)?;

            if let Object::ReturnValue(value) = result {
                return Ok(*value);
            }
        }
        Ok(result)
    }

    fn eval_statement(&mut self, stmt: &ast::Statement) -> Result<Object, MonkeyError> {
        match stmt {
            ast::Statement::Expression(expr) => self.eval_expression(expr),
            ast::Statement::Block(statements) => self.eval_block_statement(statements),
            ast::Statement::Return(expr) => {
                let obj = self.eval_expression(expr)?;
                Ok(Object::ReturnValue(Box::new(obj)))
            },
            ast::Statement::Let{ident, value} => {
                if let ast::Expression::Ident(ident) = ident {
                    let value = self.eval_expression(value)?;
                    self.store.insert(ident.to_owned(), value);
                    Ok(Object::Null)
                } else {
                    unreachable!()
                }
            },
        }
    }

    fn eval_block_statement(&mut self, stmts: &[ast::Statement]) -> Result<Object, MonkeyError> {
        let mut result = Object::Null;
    
        for stmt in stmts.iter() {
            result = self.eval_statement(stmt)?;
    
            if let Object::ReturnValue(_) = result {
                return Ok(result);
            }
        }
        Ok(result)
    }
    
    fn eval_expressions(&mut self, exprs: &[ast::Expression]) -> Result<Vec<Object>, MonkeyError> {
        let mut result = Vec::new();
        for expr in exprs.iter() {
            result.push(self.eval_expression(expr)?);
        }
        Ok(result)
    }

    fn eval_expression(&mut self, expr: &ast::Expression) -> Result<Object, MonkeyError> {
        match expr {
            ast::Expression::Integer(value) => Ok(Object::Integer(*value)),
            ast::Expression::Bool(value)    => Ok(Object::Bool(*value)),
            ast::Expression::Prefix{op, right} => {
                let right = self.eval_expression(right)?;
                eval_prefix_expression(op, right)
            },
            ast::Expression::Infix{op, left, right} => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                eval_infix_expression(op, left, right)
            },
            ast::Expression::If{condition, consequence, alternative, ..} => {
                if self.eval_expression(condition)?.is_truthy() {
                    self.eval_statement(consequence)
                } else {
                    match alternative {
                        Some(alt) => self.eval_statement(alt),
                        None      => Ok(Object::Null)
                    }
                }
            },
            ast::Expression::Ident(ident) => match self.store.get(ident) {
                Some(value) => Ok(value.clone()),
                None         => Err(MonkeyError::IdentifierNotFound(ident.to_owned())),
            },
            ast::Expression::Function{parameters, body} => {
                Ok(Object::Function{parameters: parameters.clone(), body: *body.clone(), env: self.clone()})
            },
            ast::Expression::Call{function, arguments} => {
                let function = self.eval_expression(function)?;
                let args = self.eval_expressions(arguments)?;
                
                apply_function(function, args)
            },
        }
    }
}

fn eval_prefix_expression(op: &operator::Prefix, right: Object) -> Result<Object, MonkeyError> {
    match op {
        operator::Prefix::Bang => match right {
            Object::Bool(value) => Ok(Object::Bool(!value)),
            Object::Null => Ok(Object::Bool(true)),
            _ => Ok(Object::Bool(false)),
        },
        operator::Prefix::Minus => match right {
            Object::Integer(value) => Ok(Object::Integer(- value)),
            _ => Ok(Object::Null),
        },
    }
}

fn eval_infix_expression(op: &operator::Infix, left: Object, right: Object) -> Result<Object, MonkeyError> {
    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => {
            match op {
                operator::Infix::Plus     => Ok(Object::Integer(left + right)),
                operator::Infix::Minus    => Ok(Object::Integer(left - right)),
                operator::Infix::Asterisk => Ok(Object::Integer(left * right)),
                operator::Infix::Slash    => Ok(Object::Integer(left / right)),
                operator::Infix::Eq       => Ok(Object::Bool(left == right)),
                operator::Infix::NotEq    => Ok(Object::Bool(left != right)),
                operator::Infix::LT       => Ok(Object::Bool(left < right)),
                operator::Infix::GT       => Ok(Object::Bool(left > right)),
            }
        },
        (Object::Bool(left), Object::Bool(right)) => {
            match op {
                operator::Infix::Eq    => Ok(Object::Bool(left == right)),
                operator::Infix::NotEq => Ok(Object::Bool(left != right)),
                op  => Err(MonkeyError::UnknownOperator(ObjectType::Bool, *op, ObjectType::Bool)),
            }
        },
        (Object::Integer(_), Object::Bool(_)) => {
            Err(MonkeyError::TypeMismatch(ObjectType::Integer, *op, ObjectType::Bool)) 
        },
        (Object::Bool(_), Object::Integer(_)) => {
            Err(MonkeyError::TypeMismatch(ObjectType::Bool, *op, ObjectType::Integer)) 
        },
        _ => Ok(Object::Null)
    }
}

fn apply_function(function: Object, args: Vec<Object>) -> Result<Object, MonkeyError> {
    if let Object::Function{parameters, body, env} = function {
        let mut env = env.virtual_environment();
        for (ident, arg) in parameters.iter().zip(args.iter()) {
            if let ast::Expression::Ident(ident) = ident {
                env.store.insert(ident.to_owned(), arg.clone());
            } 
        }
        match env.eval_statement(&body)? {
            Object::ReturnValue(obj) => Ok(*obj),
            obj => Ok(obj),
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast,
        lexer::Lexer,
        parser::Parser,
        object::Object,
        eval::Environment,
    };

    #[test]
    fn eval_expression() {
        let probrem = [
            ("5", Object::Integer(5)),
            ("true", Object::Bool(true)),
            ("!true", Object::Bool(false)),
            ("!5", Object::Bool(false)),
            ("5 + 5 * 10", Object::Integer(55)),
            ("24 / 2 == 12", Object::Bool(true)),
            ("(2*3*4 == 24) == true", Object::Bool(true)),
            ("if (true) { 12; } else { 13; }", Object::Integer(12)),
            ("if (0 == 1) { 42; }", Object::Null),
            ("return 10; 43;", Object::Integer(10)),
            ("if (true) { if (true) { return 10; } return 12; } else { 14; }", 
                Object::Integer(10)),
            ("let x = 5;", Object::Null),
            ("let x = 5; x*2;", Object::Integer(10)),
        ];

        for (input, answer) in probrem.iter() {
            let solution = eval(input);
            assert_eq!(&solution, answer);
        }
    }

    fn eval(input: &str) -> Object {
        let mut env = Environment::new();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("Failed to parse!");

        env.eval(&program).expect("Failed to evaluate!")
    }

    #[test]
    fn eval_function() {
        let input = "fn(x) { x + 2; };";
        let obj = eval(input);

        if let Object::Function{parameters, body, env} = obj {
            assert_eq!(env, Environment::new());
            if let ast::Expression::Ident(ident) = &parameters[0] {
                assert_eq!( ident, "x" );
                if let ast::Statement::Block(blocks) = body {
                    assert_eq!(format!("{}", blocks[0]), "(x+2);");
                }
            } else {
                panic!("");
            }
        } else {
            panic!("");
        }

    }
}
