use crate::ast::{Binary, Expr, Grouping, Literal, Unary};

fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let expr_strings = exprs
        .iter()
        .map(|&expr| print_ast(expr))
        .collect::<Vec<_>>()
        .join(" ");
    format!("({name} {expr_strings})")
}

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(Binary {
            left,
            operator,
            right,
        }) => parenthesize(operator.lexeme.as_str(), vec![left, right]),
        Expr::Grouping(Grouping { expression }) => parenthesize("group", vec![expression]),
        Expr::Literal(Literal { value }) => format!("{value:?}"),
        Expr::Unary(Unary { operator, right }) => {
            parenthesize(operator.lexeme.as_str(), vec![right])
        }
    }
}
