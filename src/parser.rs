use crate::ast::{Ctx, Expr, Mutable, Stmt};
use crate::token::{Token, TokenType};

type ParserResult<T> = Result<T, String>;

#[derive(Debug)]
struct ParserState {
    current: usize,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState { current: 0 }
    }
}

pub struct Parser<'a> {
    source_name: &'a str,
    prompt_mode: bool,
    tokens: &'a Vec<Token<'a>>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>, source_name: &'a str, prompt_mode: bool) -> Self {
        Parser {
            source_name,
            prompt_mode,
            tokens,
            state: ParserState::new(),
        }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt<'a>>> {
        let mut stmts = Vec::<Stmt<'a>>::new();
        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(stmts)
    }

    fn statement(&mut self) -> ParserResult<Stmt<'a>> {
        if self.match_ttokens(&vec![&TokenType::Let]) {
            return self.let_statement();
        }

        if self.match_ttokens(&vec![&TokenType::PrintLn]) {
            return self.print_statement();
        }

        if self.match_ttokens(&vec![&TokenType::LeftBrace]) {
            return self.block_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> ParserResult<Stmt<'a>> {
        match self.expr() {
            Ok(expr) => {
                if self.match_ttokens(&vec![&TokenType::Semicolon]) || self.prompt_mode {
                    Ok(Stmt::PrintLn(
                        Ctx::from_token(self.previous()),
                        Box::new(expr),
                    ))
                } else {
                    Err(Parser::make_parse_error_message(
                        &self,
                        self.peek(),
                        "Expect ';' after statement",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn let_statement(&mut self) -> ParserResult<Stmt<'a>> {
        let is_mutable = if self.peek().ttype == TokenType::Mut {
            self.advance();
            true
        } else {
            false
        };

        if self.peek().ttype == TokenType::Identifier {
            let identifier = self.advance().clone();

            if self.match_ttokens(&vec![&TokenType::Equal]) {
                match self.expr() {
                    Ok(expr) => {
                        if self.match_ttokens(&vec![&TokenType::Semicolon]) || self.prompt_mode {
                            Ok(Stmt::Let(
                                Ctx::from_token(&identifier),
                                identifier,
                                Mutable(is_mutable),
                                Box::new(expr),
                            ))
                        } else {
                            Err(Parser::make_parse_error_message(
                                &self,
                                self.peek(),
                                "Expect ';' after expression",
                            ))
                        }
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(Parser::make_parse_error_message(
                    &self,
                    self.peek(),
                    "Expect initialization",
                ))
            }
        } else {
            Err(Parser::make_parse_error_message(
                &self,
                self.peek(),
                "Expect identifier after let",
            ))
        }
    }

    fn block_statement(&mut self) -> ParserResult<Stmt<'a>> {
        let mut stmts = Vec::new();
        let ctx = Ctx::from_token(self.previous());
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => stmts.push(Box::new(stmt)),
                Err(e) => return Err(e),
            }
        }

        if !self.match_ttokens(&vec![&TokenType::RightBrace]) {
            return Err(Parser::make_parse_error_message(
                &self,
                self.peek(),
                "Expect '}' after block",
            ));
        }

        Ok(Stmt::Block(ctx, stmts))
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt<'a>> {
        match self.expr() {
            Ok(expr) => {
                if self.match_ttokens(&vec![&TokenType::Semicolon]) || self.prompt_mode {
                    Ok(Stmt::Expression(
                        Ctx::from_token(self.previous()),
                        Box::new(expr),
                    ))
                } else {
                    Err(Parser::make_parse_error_message(
                        &self,
                        self.peek(),
                        "Expect ';' after expression",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn expr(&mut self) -> ParserResult<Expr<'a>> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr<'a>> {
        let expr = match self.equality() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        if self.match_ttokens(&vec![&TokenType::Equal]) {
            let toke_eq = self.previous().clone();
            let value = match self.assignment() {
                Ok(value) => value,
                Err(e) => return Err(e),
            };

            match expr {
                Expr::Variable(ctx, token) => {
                    let name = token.clone();
                    Ok(Expr::Assign(ctx.clone(), name, Box::new(value.clone())))
                }
                _ => Err(Parser::make_parse_error_message(
                    &self,
                    &toke_eq,
                    "Invalid assignment value",
                )),
            }
        } else {
            Ok(expr)
        }
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
            TokenType::False | TokenType::True | TokenType::Nil => {
                let token = self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(token),
                    self.previous().clone(),
                ))
            }
            TokenType::Number(_) => {
                let token = self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(token),
                    self.previous().clone(),
                ))
            }
            TokenType::String(_) => {
                let token = self.advance();
                Ok(Expr::Literal(
                    Ctx::from_token(token),
                    self.previous().clone(),
                ))
            }
            TokenType::LeftParen => {
                let token = self.advance().clone();

                match self.expr() {
                    Ok(expr) => {
                        if self.match_ttokens(&vec![&TokenType::RightParen]) {
                            Ok(Expr::Grouping(Ctx::from_token(&token), Box::new(expr)))
                        } else {
                            Err(Parser::make_parse_error_message(
                                &self,
                                self.peek(),
                                "Expect ')' after expression",
                            ))
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            TokenType::Identifier => {
                let token = self.advance();
                Ok(Expr::Variable(Ctx::from_token(token), token.clone()))
            }
            _ => Err(Parser::make_parse_error_message(
                &self,
                self.peek(),
                "Expect expression",
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

        self.tokens[self.state.current].ttype == *ttype
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
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.state.current]
    }

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
                | TokenType::PrintLn
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn make_parse_error_message(parser: &Parser<'a>, token: &Token<'a>, message: &str) -> String {
        match token.ttype {
            TokenType::Eof => {
                format!(
                    "File \"<{}>\", line {}, in <root>\n\tError: {} at end",
                    parser.source_name, token.line, message
                )
            }
            _ => format!(
                "File \"<{}>\", line {}, in <root>\n\tError at '{}': {}",
                parser.source_name, token.line, token.lexeme, message
            ),
        }
    }
}
