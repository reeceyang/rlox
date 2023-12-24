use crate::{
    ast::{Binary, Expr, Expression, Grouping, Literal, Print, Stmt, Unary},
    scanner::{Object, Token, TokenType},
    Lox,
};

pub struct Parser<'a> {
    lox: &'a mut Lox,
    tokens: &'a Vec<Token>,
    current: usize,
}

pub struct ParseError;

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>, lox: &'a mut Lox) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
            lox,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?)
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token_types(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_owned())?;
        Ok(Stmt::Print(Print { expression: value }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after expression.".to_owned(),
        )?;
        Ok(Stmt::Expression(Expression { expression: expr }))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        while self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator: operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_types(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal {
                value: Object::Bool(false),
            }));
        }
        if self.match_token_types(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal {
                value: Object::Bool(true),
            }));
        }
        if self.match_token_types(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal { value: Object::Nil }));
        }
        if self.match_token_types(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal.clone().unwrap(),
            }));
        }
        if self.match_token_types(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            return match self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_owned(),
            ) {
                Ok(_) => Ok(Expr::Grouping(Grouping {
                    expression: Box::new(expr),
                })),
                Err(e) => Err(e),
            };
        }
        Err(self.error(self.peek().clone(), "Expect expression.".to_owned()))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        let token = self.peek().clone();
        Err(self.error(token, message))
    }

    fn error(&mut self, token: Token, message: String) -> ParseError {
        self.lox.error(token, message);

        ParseError {}
    }

    fn match_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
