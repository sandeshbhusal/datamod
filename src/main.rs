#![allow(unused)]

use std::io;
use std::io::Write;

use lparser::BinExpr;
mod chumsky;

fn main() {
}

// Lexer and parser for our lisp repl.
mod lparser {
    use core::panic;

    #[derive(Debug)]
    pub enum Operation {
        ADD,
        SUB,
        MUL,
        DIV,
        NEG,
    }

    #[derive(Debug)]
    pub enum Expr {
        BinExpr(BinExpr),
        UnaryExpr(UnaryExpr),
        Number(i32),
    }

    #[derive(Debug)]
    pub struct BinExpr {
        op: Operation,
        left: Box<Expr>,
        right: Box<Expr>,
    }

    #[derive(Debug)]
    pub struct UnaryExpr {
        op: Operation,
        operand: Box<Expr>,
    }

    impl BinExpr {
        pub(crate) fn new(op: Operation, left: Box<Expr>, right: Box<Expr>) -> BinExpr {
            BinExpr {
                op: op,
                left: left,
                right: right,
            }
        }

        fn eval(&self) -> i32 {
            let left = self.left.eval();
            let right = self.right.eval();

            match self.op {
                Operation::ADD => left + right,
                Operation::SUB => left - right,
                Operation::MUL => left * right,
                Operation::DIV => left / right,
                Operation::NEG => panic!("Negation inside a binary expr cannot happen. Unary expr evals should happen first."),
            }
        }
    }

    impl UnaryExpr {
        pub(crate) fn new(op: Operation, operand: Box<Expr>) -> UnaryExpr {
            UnaryExpr {
                op: op,
                operand: operand,
            }
        }

        fn eval(&self) -> i32 {
            match self.op {
                Operation::NEG => -self.operand.eval(),
                _ => self.operand.eval(),
            }
        }
    }

    impl Expr {
        pub fn eval(&self) -> i32 {
            match self {
                Expr::BinExpr(be) => be.eval(),
                Expr::UnaryExpr(ue) => ue.eval(),
                Expr::Number(n) => *n,
            }
        }
    }
}

mod tests {
    use crate::lparser::{self, BinExpr, UnaryExpr};

    #[test]
    fn test_eval() {
        use lparser::Expr;
        use lparser::Operation::*;

        let expr = Expr::BinExpr(BinExpr::new(
            ADD,
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        ));
        assert_eq!(3, expr.eval());

        let expr = Expr::BinExpr(BinExpr::new(
            SUB,
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        ));
        assert_eq!(-1, expr.eval());

        let expr = Expr::BinExpr(BinExpr::new(
            MUL,
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        ));
        assert_eq!(2, expr.eval());

        let expr = Expr::BinExpr(BinExpr::new(
            DIV,
            Box::new(Expr::Number(2)),
            Box::new(Expr::Number(4)),
        ));
        assert_eq!(0, expr.eval());

        let expr = Expr::UnaryExpr(UnaryExpr::new(
            NEG,
            Box::new(Expr::Number(4)),
        ));
        assert_eq!(-4, expr.eval());
        let expr = Expr::UnaryExpr(UnaryExpr::new(
            ADD,
            Box::new(Expr::Number(4)),
        ));

        // Combine two expressions, and check.
        let expr = Expr::BinExpr(BinExpr::new(
            ADD,
            Box::new(Expr::Number(1)),
            Box::new(Expr::UnaryExpr(UnaryExpr::new(
                NEG,
                Box::new(Expr::Number(2)),
            ))),
        ));
        assert_eq!(-1, expr.eval());
    }
}
