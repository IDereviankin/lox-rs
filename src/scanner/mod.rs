mod token;

use self::token::{Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    stream: std::iter::Peekable<std::str::Chars<'a>>,

    start: i32,
    current: i32,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let stream = source.chars().peekable();

        Self {
            source,
            tokens: vec![],
            stream,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while let Some(c) = self.stream.next() {
            self.start = self.current;

            use TokenKind::*;
            match c {
                '(' => self.addToken(LeftParen),
                ')' => self.addToken(RightParen),
                '{' => self.addToken(LeftBrace),
                '}' => self.addToken(RightBrace),
                ',' => self.addToken(Comma),
                '.' => self.addToken(Dot),
                '-' => self.addToken(Minus),
                '+' => self.addToken(Plus),
                ';' => self.addToken(Semicolon),
                '*' => self.addToken(Star),
                '!' => {
                    let tok = if self.expect_next('=') {
                        BangEqual
                    } else {
                        Bang
                    };
                    self.addToken(tok);
                }
                '=' => {
                    let tok = if self.expect_next('=') {
                        EqualEqual
                    } else {
                        Equal
                    };
                    self.addToken(tok);
                }
                '<' => {
                    let tok = if self.expect_next('=') {
                        LessEqual
                    } else {
                        Less
                    };
                    self.addToken(tok);
                }
                '>' => {
                    let tok = if self.expect_next('=') {
                        GreaterEqual
                    } else {
                        Greater
                    };
                    self.addToken(tok);
                }
                '/' => {
                    if self.expect_next('/') {
                        while let Some(c) = self.stream.peek() {
                            if *c == '\n' { break }
                            self.stream.next();
                        }
                    } else {
                        self.addToken(Slash);
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => panic!("Unexpected character at line {}", self.line),
            }
        }

        self.tokens
    }

    fn addToken(&mut self, token: TokenKind) {
        self.tokens.push(Token::new(token, self.line))
    }

    fn expect_next(&mut self, a: char) -> bool {
        if let Some(b) = self.stream.peek() {
            if a == *b {
                self.stream.next();
                return true;
            }
        }

        false
    }
}
