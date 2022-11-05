#[derive(Debug, Clone)]
pub enum TokenType<'a> {
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
    String(&'a str),
    Number(f64),

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
    Let,
    Const,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(ttype: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            line,
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ttype, self.lexeme)
    }
}

impl<'a> std::fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
