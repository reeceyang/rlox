use crate::{
    ast::Expr,
    scanner::{Object, Token, TokenType},
    Lox,
};

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}

fn evaluate(expr: Expr) -> Result<Object, RuntimeError> {
    match expr {
        Expr::Binary(e) => {
            let left = evaluate(*e.left)?;
            let right = evaluate(*e.right)?;

            match e.operator.token_type {
                TokenType::Minus => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::F64(left_value - right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::Slash => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::F64(left_value / right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::Star => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::F64(left_value * right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::Plus => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::F64(left_value + right_value)),
                        _ => Err(addition_operands_error(e.operator)),
                    },
                    Object::Str(left_value) => match right {
                        Object::Str(right_value) => {
                            Ok(Object::Str(format!("{}{}", left_value, right_value)))
                        }
                        _ => Err(addition_operands_error(e.operator)),
                    },
                    _ => Err(addition_operands_error(e.operator)),
                },
                TokenType::Greater => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::Bool(left_value > right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::GreaterEqual => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::Bool(left_value >= right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::Less => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::Bool(left_value < right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::LessEqual => match left {
                    Object::F64(left_value) => match right {
                        Object::F64(right_value) => Ok(Object::Bool(left_value <= right_value)),
                        _ => Err(number_operands_error(e.operator)),
                    },
                    _ => Err(number_operands_error(e.operator)),
                },
                TokenType::BangEqual => Ok(Object::Bool(!is_equal(left, right))),
                TokenType::EqualEqual => Ok(Object::Bool(is_equal(left, right))),
                _ => Ok(Object::Nil), // unreachable
            }
        }
        Expr::Grouping(e) => evaluate(*e.expression),
        Expr::Literal(e) => Ok(e.value),
        Expr::Unary(e) => {
            let right = evaluate(*e.right)?;

            match e.operator.token_type {
                TokenType::Minus => match right {
                    Object::F64(value) => Ok(Object::F64(-value)),
                    _ => Err(number_operand_error(e.operator)),
                },
                TokenType::Bang => Ok(Object::Bool(is_truthy(right))),
                _ => Ok(Object::Nil), // unreachable
            }
        }
    }
}

pub fn interpret(expr: Expr, lox: &mut Lox) {
    match evaluate(expr) {
        Ok(value) => println!("{}", stringify(value)),
        Err(e) => lox.runtime_error(e),
    }
}

fn stringify(object: Object) -> String {
    match object {
        Object::Str(value) => value,
        Object::F64(value) => match value.to_string().strip_suffix(".0") {
            // strip the trailing .0 if value is an integer
            Some(s) => s.to_owned(),
            None => value.to_string(),
        },

        Object::Bool(value) => value.to_string(),
        Object::Nil => "nil".to_owned(),
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Bool(value) => value,
        Object::Nil => false,
        _ => true,
    }
}

fn is_equal(a: Object, b: Object) -> bool {
    match a {
        Object::Nil => match b {
            Object::Nil => true,
            _ => false,
        },
        _ => a == b,
    }
}

fn addition_operands_error(token: Token) -> RuntimeError {
    RuntimeError::new(
        token,
        "Operands must be two numbers or two strings.".to_owned(),
    )
}

fn number_operands_error(token: Token) -> RuntimeError {
    RuntimeError::new(token, "Operands must be numbers.".to_owned())
}

fn number_operand_error(token: Token) -> RuntimeError {
    RuntimeError::new(token, "Operand must be a number.".to_owned())
}
