#[rustfmt::skip]
#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier(String), String(String), Number(f64),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        write!(f, "{}", match &self.kind {
            LeftParen => "(".to_string(),
            RightParen => ")".to_string(),
            LeftBrace => "{".to_string(),
            RightBrace => "}".to_string(),
            Comma => ",".to_string(),
            Dot => ".".to_string(),
            Minus => "-".to_string(),
            Plus => "+".to_string(),
            Semicolon => ";".to_string(),
            Slash => "/".to_string(),
            Star => "*".to_string(),
            Bang => "!".to_string(),
            BangEqual => "!=".to_string(),
            Equal => "=".to_string(),
            EqualEqual => "==".to_string(),
            Greater => ">".to_string(),
            GreaterEqual => ">=".to_string(),
            Less => "<".to_string(),
            LessEqual => "<=".to_string(),
            Identifier(id) => id.to_string(),
            String(s) => format!("\"{}\"", s),
            Number(n) => format!("{}", n),
            And => "and".to_string(),
            Class => "class".to_string(),
            Else => "else".to_string(),
            False => "false".to_string(),
            Fun => "fun".to_string(),
            For => "for".to_string(),
            If => "if".to_string(),
            Nil => "nil".to_string(),
            Or => "or".to_string(),
            Print => "print".to_string(),
            Return => "return".to_string(),
            Super => "super".to_string(),
            This => "this".to_string(),
            True => "true".to_string(),
            Var => "var".to_string(),
            While => "while".to_string(),
            Eof => "\n".to_string(),
        }).unwrap();
        Ok(())
    }
}