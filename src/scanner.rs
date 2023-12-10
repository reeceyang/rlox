use core::fmt;
use std::string::String;

use phf::phf_map;

use crate::Lox;

#[derive(Debug, Clone, Copy)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub enum Object {
    Str(String),
    F64(f64),
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: i32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" =>    TokenType::And,
    "class" =>  TokenType::Class,
    "else" =>   TokenType::Else,
    "false" =>  TokenType::False,
    "for" =>    TokenType::For,
    "fun" =>    TokenType::Fun,
    "if" =>     TokenType::If,
    "nil" =>    TokenType::Nil,
    "or" =>     TokenType::Or,
    "print" =>  TokenType::Print,
    "return" => TokenType::Return,
    "super" =>  TokenType::Super,
    "this" =>   TokenType::This,
    "true" =>   TokenType::True,
    "var" =>    TokenType::Var,
    "while" =>  TokenType::While,
};

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    lox: &'a mut Lox,
}

impl Scanner<'_> {
    pub fn new<'a>(source: &'a String, lox: &'a mut Lox) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            lox,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None)
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None)
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None)
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            '"' => self.string(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_ascii_alphabetic() => self.identifier(),
            _ => self.lox.error(self.line, "Unexpected character."),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        match c {
            Some(c) => c,
            None => panic!("Scanner failed at line {}", self.line),
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current);
        match c {
            Some(c) => {
                if c != expected {
                    false
                } else {
                    self.current += 1;
                    true
                }
            }
            None => panic!("Scanner failed at line {}", self.line),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1 - (self.start + 1))
            .collect();
        self.add_token(TokenType::String, Some(Object::Str(value)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.add_token(
            TokenType::Number,
            Some(Object::F64(value.as_str().parse::<f64>().unwrap())),
        )
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        let token_type = if KEYWORDS.contains_key(text.as_str()) {
            *KEYWORDS.get(text.as_str()).unwrap()
        } else {
            TokenType::Identifier
        };

        self.add_token(token_type, None)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        })
    }
}
