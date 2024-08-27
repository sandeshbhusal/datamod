use std::io;
use std::io::Write;

fn main() {
    let repl_prompt: String = String::from(">> ");

    loop {
        print!("{repl_prompt}");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        print!("{buffer}");
    }
}

// Lexer and parser for our lisp repl.
mod lparser {
    use core::panic;

    enum Operation {
        ADD,
        SUB,
        MUL,
        DIV,
        NEG,
    }

    enum Expr {
        BinExpr(BinExpr),
        UnaryExpr(UnaryExpr),
        Number(i32),
    }

    struct BinExpr {
        op: Operation,
        left: Box<Expr>,
        right: Box<Expr>,
    }

    struct UnaryExpr {
        op: Operation,
        operand: Box<Expr>,
    }

    impl BinExpr {
        fn new(op: Operation, left: Box<Expr>, right: Box<Expr>) -> BinExpr {
            BinExpr {
                op: op,
                left: left,
                right: right,
            }
        }

        pub fn eval(&self) -> i32 {
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
        fn new(op: Operation, operand: Box<Expr>) -> UnaryExpr {
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
        fn eval(&self) -> i32 {
            match self {
                Expr::BinExpr(be) => be.eval(),
                Expr::UnaryExpr(ue) => ue.eval(),
                Expr::Number(n) => *n,
            }
        }
    }
}
