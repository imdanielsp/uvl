#[derive(Debug, Clone, PartialEq)]
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
    Mut,
    Or,
    PrintLn,
    Return,
    Super,
    This,
    True,
    Let,
    Const,
    While,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
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
