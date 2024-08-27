#![allow(unused)]

use std::io;
use std::io::Write;

mod chumsky;

fn main() {}

// Lexer and parser for our lisp repl.
mod lparser {
    use core::panic;

    #[derive(Debug)]
    pub enum Expr {
        // Binary expression
        AddExpr(Box<Expr>, Box<Expr>),
        SubExpr(Box<Expr>, Box<Expr>),
        MulExpr(Box<Expr>, Box<Expr>),
        DivExpr(Box<Expr>, Box<Expr>),

        // Unary expression
        NegExpr(Box<Expr>),

        // Number
        Number(i32),
    }

    impl Expr {
        pub fn eval(&self) -> i32 {
            match self {
                Expr::AddExpr(left, right) => left.eval() + right.eval(),
                Expr::SubExpr(left, right) => left.eval() - right.eval(),
                Expr::MulExpr(left, right) => left.eval() * right.eval(),
                Expr::DivExpr(left, right) => left.eval() / right.eval(),
                Expr::NegExpr(expr) => -expr.eval(),
                Expr::Number(n) => *n,
            }
        }
    }
}
