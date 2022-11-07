use crate::ast::{Ctx, Expr, Stmt};
use crate::token::{Token, TokenType};

type ParserResult<T> = Result<T, String>;

struct ParserState {
    current: usize,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState { current: 0 }
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            state: ParserState::new(),
        }
    }

    pub fn parse(&mut self) -> ParserResult<Expr> {
        self.expr()
    }

    // todo!()
    // pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
    //     let mut stmts = Vec::<Stmt>::new();
    //     while !self.is_at_end() {
    //         match self.statement() {
    //             Ok(stmt) => stmts.push(stmt),
    //             Err(e) => return Err(e),
    //         }
    //     }
    //     Ok(stmts)
    // }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().ttype {
                TokenType::Semicolon => {
                    return;
                }
                _ => (),
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.match_ttokens(&vec![&TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        Err("TODO".to_string())
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        Err("TODO".to_string())
    }

    // TODO: this is temp public
    pub fn expr(&mut self) -> ParserResult<Expr<'a>> {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult<Expr<'a>> {
        match self.comparison() {
            Ok(expr) => {
                while self.match_ttokens(&vec![&TokenType::BangEqual, &TokenType::EqualEqual]) {
                    let operator = self.previous().clone();
                    if let Ok(right) = self.comparison() {
                        return Ok(Expr::Binary(
                            Ctx::from_token(self.previous()),
                            Box::new(expr),
                            operator,
                            Box::new(right),
                        ));
                    };
                }

                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn comparison(&mut self) -> ParserResult<Expr<'a>> {
        match self.term() {
            Ok(expr) => {
                while self.match_ttokens(&vec![
                    &TokenType::Greater,
                    &TokenType::GreaterEqual,
                    &TokenType::Less,
                    &TokenType::LessEqual,
                ]) {
                    let operator = self.previous().clone();
                    if let Ok(right) = self.term() {
                        return Ok(Expr::Binary(
                            Ctx::from_token(self.previous()),
                            Box::new(expr),
                            operator,
                            Box::new(right),
                        ));
                    }
                }

                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn term(&mut self) -> ParserResult<Expr<'a>> {
        match self.factor() {
            Ok(expr) => {
                while self.match_ttokens(&vec![&TokenType::Minus, &TokenType::Plus]) {
                    let operator = self.previous().clone();
                    if let Ok(right) = self.factor() {
                        return Ok(Expr::Binary(
                            Ctx::from_token(self.previous()),
                            Box::new(expr),
                            operator,
                            Box::new(right),
                        ));
                    }
                }

                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn factor(&mut self) -> ParserResult<Expr<'a>> {
        match self.unary() {
            Ok(expr) => {
                while self.match_ttokens(&vec![&TokenType::Slash, &TokenType::Star]) {
                    let operator = self.previous().clone();
                    if let Ok(right) = self.unary() {
                        return Ok(Expr::Binary(
                            Ctx::from_token(self.previous()),
                            Box::new(expr),
                            operator,
                            Box::new(right),
                        ));
                    }
                }

                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn unary(&mut self) -> ParserResult<Expr<'a>> {
        if self.match_ttokens(&vec![&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            if let Ok(right) = self.unary() {
                return Ok(Expr::Unary(
                    Ctx::from_token(self.previous()),
                    operator,
                    Box::new(right),
                ));
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr<'a>, String> {
        match self.peek().ttype {
            TokenType::False | TokenType::True => {
                self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(self.previous()),
                    self.previous().clone(),
                ))
            }
            TokenType::Number(_) => {
                self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(self.previous()),
                    self.previous().clone(),
                ))
            }
            TokenType::String(_) => {
                self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(self.previous()),
                    self.previous().clone(),
                ))
            }
            TokenType::LeftParen => {
                self.advance();

                match self.expr() {
                    Ok(expr) => {
                        if self.match_ttokens(&vec![&TokenType::RightParen]) {
                            Ok(Expr::Grouping(
                                Ctx::from_token(self.previous()),
                                Box::new(expr),
                            ))
                        } else {
                            Err(Parser::make_parse_error_message(
                                self.peek(),
                                "Expect ')' after expression",
                            ))
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err(Parser::make_parse_error_message(
                self.peek(),
                "Expect expression.",
            )),
        }
    }

    fn match_ttokens(&mut self, ttypes: &[&'a TokenType<'a>]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, ttype: &'a TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.tokens[self.state.current].ttype == ttype.clone()
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.state.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.state.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() == self.state.current
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.state.current]
    }

    fn make_parse_error_message(token: &Token<'a>, message: &str) -> String {
        match token.ttype {
            TokenType::Eof => {
                format!("[line {}] Error {}: {}", token.line, " at end", message)
            }
            _ => format!(
                "[line {}] Error at '{}': {}",
                token.line, token.lexeme, message
            ),
        }
    }
}
