use std::collections::HashMap;

use crate::token::{Token, TokenType};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType<'static>> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("mut", TokenType::Mut);
        keywords.insert("or", TokenType::Or);
        keywords.insert("println", TokenType::PrintLn);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("let", TokenType::Let);
        keywords.insert("const", TokenType::Const);
        keywords.insert("while", TokenType::While);
        keywords
    };
}

struct LexerState {
    start: usize,
    current: usize,
    line: usize,
}

impl LexerState {
    pub fn new() -> Self {
        LexerState {
            start: 0,
            current: 0,
            line: 0,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: vec![],
            state: LexerState::new(),
        }
    }

    pub fn scan(&mut self) -> &Vec<Token<'a>> {
        while !self.is_at_end() {
            self.state.start = self.state.current;
            self.scan_token();
        }

        // Set end of line
        self.tokens
            .push(Token::new(TokenType::Eof, "", self.state.line));
        &self.tokens
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.state.current >= self.source.len()
    }

    pub(crate) fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let t = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t);
            }
            '=' => {
                let t = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t);
            }
            '<' => {
                let t = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t);
            }
            '>' => {
                let t = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t);
            }
            '/' => {
                if self.match_next('/') {
                    // We have a comment that needs to be consumed until the end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            c if c.is_digit(10) => self.scan_number(),
            c if c.is_alphabetic() => self.scan_identifier(),
            '"' => self.scan_string(),
            // Increment new line state
            '\n' => self.state.line += 1,
            // Ignore whitespace
            ' ' | '\r' | '\t' => (),
            _ => println!("[TODO make this a Err(..)] -- Unexpected character {}", c)
            // _ => self
            //     .logger
            //     .error(self.state.line, &format!("Unexpected character {}", c)),
        }
    }

    pub(crate) fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.state.current).expect(&format!(
            "Unexpected error in the lexer reading char @ index {}",
            self.state.current
        ));
        self.state.current += 1;
        c
    }

    pub(crate) fn add_token(&mut self, ttype: TokenType<'a>) {
        let text = &self.source[self.state.start..self.state.current];
        self.tokens.push(Token::new(ttype, text, self.state.line));
    }

    pub(crate) fn match_next(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.chars().nth(self.state.current).expect(&format!(
            "Unexpected error in the lexer reading char @ index {}",
            self.state.current
        ));
        if next_char != expected_char {
            return false;
        }

        self.state.current += 1;
        true
    }

    pub(crate) fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.state.current).expect(&format!(
                "Unexpected error in the lexer reading char @ index {}",
                self.state.current
            ))
        }
    }

    pub(crate) fn peek_next(&mut self) -> char {
        if self.state.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(self.state.current + 1)
                .expect(&format!(
                    "Unexpected error in the lexer reading char @ index {}",
                    self.state.current + 1
                ))
        }
    }

    pub(crate) fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.state.line += 1;
            }
            self.advance();
        }

        // Consume the closing "
        self.advance();

        // Extract the string literal (without the surrounding quotes)
        let str_value = &self.source[self.state.start + 1..self.state.current - 1];
        self.add_token(TokenType::String(str_value));
    }

    pub(crate) fn scan_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Check if this is a fraction
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consumes the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal = &self.source[self.state.start..self.state.current];
        let value = literal.parse::<f64>().expect(&format!(
            "Interpreter internal error: failed to parse {} as a f64",
            literal
        ));
        self.add_token(TokenType::Number(value))
    }

    pub(crate) fn scan_identifier(&mut self) {
        while self.peek().is_alphabetic() || self.peek().is_alphanumeric() {
            self.advance();
        }

        let identifier = &self.source[self.state.start..self.state.current];
        let ttype = KEYWORDS
            .get(&identifier)
            .unwrap_or(&TokenType::Identifier)
            .clone();
        self.add_token(ttype);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_integer() {
        let mut lex = Lexer::new("1");

        let tokens = lex.scan();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].lexeme, "1");
        assert!(matches!(tokens[0].ttype, TokenType::Number(1.0)));
    }

    #[test]
    fn t_a() {
        assert!(true);
    }

    #[test]
    fn t_b() {
        assert!(false);
    }
}
