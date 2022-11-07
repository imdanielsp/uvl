use crate::ast::Ctx;
use crate::common::make_error_msg;
use crate::token::TokenType;

pub type UvlResult = Result<UvlValue, UvlError>;

#[derive(Debug)]
pub enum UvlError {
    RuntimeError(String),
    UnsupportedOperator(String),
    ParserError(String),
}

impl std::fmt::Display for UvlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeError(s) => write!(f, "{}", s),
            Self::ParserError(s) => write!(f, "{}", s),
            Self::UnsupportedOperator(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UvlValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil(()),
}

impl std::fmt::Display for UvlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil(_) => write!(f, "()"),
        }
    }
}

impl UvlValue {
    pub fn type_str(&self) -> &'static str {
        match self {
            UvlValue::String(_) => "String",
            UvlValue::Number(_) => "Number",
            UvlValue::Bool(_) => "Bool",
            UvlValue::Nil(_) => "Nil",
        }
    }

    pub fn apply_operator(&self, ctx: &Ctx, op: &TokenType, rhs: Option<&UvlValue>) -> UvlResult {
        if let Some(rhs) = rhs {
            match op {
                TokenType::EqualEqual => self.eq(ctx, rhs),
                TokenType::BangEqual => self.neq(ctx, rhs),
                TokenType::Greater => self.gt(ctx, rhs),
                TokenType::GreaterEqual => self.ge(ctx, rhs),
                TokenType::Less => self.lt(ctx, rhs),
                TokenType::LessEqual => self.le(ctx, rhs),
                TokenType::Plus => self.add(ctx, rhs),
                TokenType::Minus => self.minus(ctx, rhs),
                TokenType::Star => self.multi(ctx, rhs),
                TokenType::Slash => self.divide(ctx, rhs),
                _ => Err(UvlError::UnsupportedOperator(make_error_msg(
                    ctx,
                    format!("{:?} is not supported", op),
                ))),
            }
        } else {
            match op {
                TokenType::Minus => {
                    if let UvlValue::Number(num) = self {
                        Ok(UvlValue::Number(-num))
                    } else {
                        Err(UvlError::UnsupportedOperator(make_error_msg(
                            ctx,
                            format!(
                                "Operator '-' is not supported for {} of type {}",
                                self,
                                self.type_str()
                            ),
                        )))
                    }
                }
                _ => Err(UvlError::UnsupportedOperator(make_error_msg(
                    ctx,
                    format!("Unsupported operator {:?}", op),
                ))),
            }
        }
    }

    fn add(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Number(lhs_num + rhs_num))
        } else if let (UvlValue::String(lhs_str), UvlValue::String(rhs_str)) = (self, rhs) {
            Ok(UvlValue::String(format!("{}{}", lhs_str, rhs_str)))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '+' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn minus(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Number(lhs_num - rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '-' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn multi(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Number(lhs_num * rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '*' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn divide(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            if *rhs_num != 0.0 {
                Ok(UvlValue::Number(lhs_num / rhs_num))
            } else {
                Err(UvlError::RuntimeError(make_error_msg(
                    ctx,
                    format!("Division by zero: {}/{}", lhs_num, rhs_num),
                )))
            }
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '/' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn gt(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Bool(lhs_num > rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '>' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn ge(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Bool(lhs_num >= rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '>=' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn lt(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Bool(lhs_num < rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '<' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn le(&self, ctx: &Ctx, rhs: &UvlValue) -> UvlResult {
        if let (UvlValue::Number(lhs_num), UvlValue::Number(rhs_num)) = (self, rhs) {
            Ok(UvlValue::Bool(lhs_num <= rhs_num))
        } else {
            Err(UvlError::UnsupportedOperator(make_error_msg(
                ctx,
                format!(
                    "Operator '<=' is not supported for {} of type {} and {} of type {}",
                    self,
                    self.type_str(),
                    rhs,
                    rhs.type_str()
                ),
            )))
        }
    }

    fn eq(&self, _: &Ctx, rhs: &UvlValue) -> Result<UvlValue, UvlError> {
        Ok(UvlValue::Bool(self == rhs))
    }

    fn neq(&self, ctx: &Ctx, rhs: &UvlValue) -> Result<UvlValue, UvlError> {
        self.eq(ctx, rhs)
    }
}
