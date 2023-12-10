use crate::scanner::{Object, Token};

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: Object,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

// pub trait Visitor {
//     fn visit_binary(&mut self, expr: &Binary);
//     fn visit_grouping(&mut self, expr: &Grouping);
//     fn visit_literal(&mut self, expr: &Literal);
//     fn visit_unary(&mut self, expr: &Unary);
// }

// pub fn walk_expr(visitor: &mut dyn Visitor, e: &Expr) {
//     let _ = visitor;
//     match e {
//         Expr::Binary(expr) => visitor.visit_binary(&expr),
//         Expr::Grouping(expr) => visitor.visit_grouping(&expr),
//         Expr::Literal(expr) => visitor.visit_literal(&expr),
//         Expr::Unary(expr) => visitor.visit_unary(&expr),
//     }
// }
