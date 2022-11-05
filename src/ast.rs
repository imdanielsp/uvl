use crate::token::{Token, TokenType};

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(TokenType<'a>),
    Unary(Token<'a>, Box<Expr<'a>>),
}

pub fn to_string<'a>(expr: &Expr<'a>) -> String {
    match expr {
        Expr::Binary(left, op, right) => {
            format!("({} {} {})", op.lexeme, to_string(left), to_string(right))
        }
        Expr::Grouping(e) => format!("(group {})", to_string(e)),
        Expr::Literal(ttype) => match ttype {
            TokenType::String(s) => s.to_string(),
            TokenType::Number(n) => n.to_string(),
            _ => "null".to_string(),
        },
        Expr::Unary(op, e) => format!("({} {})", op.lexeme, to_string(e)),
    }
}
