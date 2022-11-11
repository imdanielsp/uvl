use crate::ast::{Ctx, Expr, Stmt};
use crate::common::make_error_msg;
use crate::envr::Environment;
use crate::token::{Token, TokenType};
use crate::value::{UvlError, UvlResult, UvlValue};

pub struct UvlInterpreter {
    had_error: bool,
    environment: Environment,
}

impl UvlInterpreter {
    pub fn new() -> UvlInterpreter {
        UvlInterpreter {
            had_error: false,
            environment: Environment::new(),
        }
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }

    pub fn run(&mut self, source: &str) -> UvlResult {
        let mut lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.scan().clone();
        let mut parser = crate::parser::Parser::new(&tokens);

        match parser.parse() {
            Ok(stmts) => self.execute(&stmts),
            Err(e) => Err(UvlError::ParserError(e)),
        }
    }

    fn execute(&mut self, stmts: &Vec<Stmt>) -> UvlResult {
        for stmt in stmts {
            match stmt {
                Stmt::Expression(_, expr) => {
                    if let Err(e) = self.eval_expr(expr) {
                        return Err(e);
                    }
                }
                Stmt::PrintLn(_, expr) => {
                    // Temporary until we get functions
                    match self.eval_expr(expr) {
                        Ok(val) => {
                            if let Err(e) = self.exec_println(&val) {
                                return Err(e);
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Stmt::Let(ctx, token, is_mutable, expr) => {
                    if self.environment.contains(token.lexeme) {
                        return Err(UvlError::NameError(make_error_msg(
                            ctx,
                            format!("Name '{}' has already been declared", token.lexeme),
                        )));
                    }

                    match self.eval_expr(expr) {
                        Ok(val) => {
                            self.environment.define(token.lexeme, is_mutable.0, val);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        Ok(UvlValue::Nil(()))
    }

    fn exec_println(&mut self, val: &UvlValue) -> UvlResult {
        println!("{}", val);
        Ok(UvlValue::Nil(()))
    }

    fn eval_expr(&mut self, expr: &Expr) -> UvlResult {
        match expr {
            Expr::Binary(ctx, left, op, right) => self.eval_bin_expr(ctx, left, &op, right),
            Expr::Grouping(_, expr) => self.eval_expr(expr),
            Expr::Unary(ctx, op, expr) => match self.eval_expr(expr) {
                Ok(expr) => expr.apply_operator(ctx, &op.ttype, None),
                Err(e) => Err(e),
            },
            Expr::Literal(_, token) => match token.ttype {
                TokenType::String(s) => Ok(UvlValue::String(s.to_string())),
                TokenType::Number(n) => Ok(UvlValue::Number(n)),
                _ => Ok(UvlValue::Nil(())),
            },
            Expr::Variable(ctx, token) => match self.environment.get(token.lexeme) {
                Some(entry) => Ok(entry.value.clone()),
                None => Err(UvlError::NameError(make_error_msg(
                    ctx,
                    format!("Name '{}' is not defined", token.lexeme),
                ))),
            },
            Expr::Assign(ctx, token, expr) => {
                if let Some(entry) = self.environment.get(token.lexeme) {
                    if !entry.is_mutable {
                        Err(UvlError::NameError(make_error_msg(
                            ctx,
                            format!("Name '{}' is immutable", token.lexeme),
                        )))
                    } else {
                        match self.eval_expr(expr) {
                            Ok(val) => {
                                self.environment.define(token.lexeme, entry.is_mutable, val);
                                Ok(entry.value.clone())
                            }
                            Err(e) => Err(e),
                        }
                    }
                } else {
                    Err(UvlError::NameError(make_error_msg(
                        ctx,
                        format!("Name '{}' is not defined", token.lexeme),
                    )))
                }
            }
        }
    }

    fn eval_bin_expr(
        &mut self,
        ctx: &Ctx,
        left: &Box<Expr>,
        op: &Token,
        right: &Box<Expr>,
    ) -> UvlResult {
        let left_val = self.eval_expr(left);
        let right_val = self.eval_expr(right);

        if left_val.is_err() {
            return left_val;
        }

        if right_val.is_err() {
            return right_val;
        }

        left_val
            .unwrap()
            .apply_operator(ctx, &op.ttype, Some(&right_val.unwrap()))
    }
}
