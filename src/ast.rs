use crate::token::Token;

#[derive(Clone, Debug, Default)]
pub struct Ctx {
    pub file: String,
    pub line: usize,
    pub module: String,
}

impl Ctx {
    pub fn from_token(token: &Token) -> Self {
        Ctx {
            file: "main.uvl".to_string(),
            line: token.line,
            module: "root".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Ctx, Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Ctx, Box<Expr<'a>>),
    Literal(Ctx, Token<'a>),
    Unary(Ctx, Token<'a>, Box<Expr<'a>>),
    Variable(Ctx, Token<'a>),
    Assign(Ctx, Token<'a>, Box<Expr<'a>>),
}

#[derive(Debug, Clone)]
pub struct Mutable(pub bool);

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Expression(Ctx, Box<Expr<'a>>),
    PrintLn(Ctx, Box<Expr<'a>>),
    Let(Ctx, Token<'a>, Mutable, Box<Expr<'a>>),
    Block(Ctx, Vec<Box<Stmt<'a>>>),
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
