use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Ctx {
    pub file: String,
    pub line: usize,
    pub module: String,
}

impl Ctx {
    pub fn new(file: String, line: usize, module: String) -> Self {
        Ctx { file, line, module }
    }

    pub fn from_token(token: &Token) -> Self {
        Ctx {
            file: "main.uvl".to_string(),
            line: token.line,
            module: "root".to_string(),
        }
    }
}

pub enum Expr<'a> {
    Binary(Ctx, Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Ctx, Box<Expr<'a>>),
    Literal(Ctx, Token<'a>),
    Unary(Ctx, Token<'a>, Box<Expr<'a>>),
}

pub enum Stmt<'a> {
    Expression(Ctx, Expr<'a>),
    Print(Ctx, Expr<'a>),
}

pub fn to_string<'a>(expr: &Expr<'a>) -> String {
    match expr {
        Expr::Binary(_, left, op, right) => {
            format!("({} {} {})", op.lexeme, to_string(left), to_string(right))
        }
        Expr::Grouping(_, e) => format!("(group {})", to_string(e)),
        Expr::Literal(_, token) => match token.ttype {
            TokenType::String(s) => s.to_string(),
            TokenType::Number(n) => n.to_string(),
            TokenType::True => "true".to_string(),
            TokenType::False => "false".to_string(),
            _ => "null".to_string(),
        },
        Expr::Unary(_, op, e) => format!("({} {})", op.lexeme, to_string(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::*, token};

    #[test]
    fn binary_to_string() {
        let val_1 = token::Token::new(token::TokenType::Number(1.0), "1.0", 0);
        let val_2 = token::Token::new(token::TokenType::Number(4.0), "4.0", 0);
        let tt_plus_op = token::TokenType::Plus;
        let op = token::Token::new(tt_plus_op, "+", 0);

        let b_expr = Expr::Binary(
            Ctx::default(),
            Box::new(Expr::Literal(Ctx::default(), val_1)),
            op,
            Box::new(Expr::Literal(Ctx::default(), val_2)),
        );

        assert_eq!(to_string(&b_expr), "(+ 1 4)");
    }
}
