use crate::ast::{Ctx, Expr};
use crate::token::{Token, TokenType};
use crate::value::{UvlError, UvlResult, UvlValue};

pub struct UvlInterpreter {
    had_error: bool,
}

impl UvlInterpreter {
    pub fn new() -> UvlInterpreter {
        UvlInterpreter { had_error: false }
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }

    pub fn run(&mut self, source: &str) -> UvlResult {
        let mut lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.scan().clone();
        let mut parser = crate::parser::Parser::new(&tokens);

        match parser.parse() {
            Ok(expr) => self.eval_expr(&expr),
            Err(e) => Err(UvlError::ParserError(e)),
        }
    }

    fn eval_expr(&self, expr: &Expr) -> UvlResult {
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
        }
    }

    fn eval_bin_expr(
        &self,
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
