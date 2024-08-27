mod tutorial {
    use std::array;

    use chumsky::prelude::*;
    use text::ident;

    use crate::lparser::Expr;

    #[derive(Debug, Clone)]
    struct ProgramUnit {
        statements: Vec<Statement>,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum BinOpr {
        Add,
        Sub,
        Mul,
        Rem,
        Div,
        BooleanCmpLess,
        BooleanCmpGreater,
        BooleanCmpEqual,
        BooleanCmpNotEqual,
        BooleanCmpLessEqual,
        BooleanCmpGreaterEqual,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum UnOpr {
        Neg,
        Not,
    }

    #[derive(Debug, Clone)]
    enum Expression {
        Integer(i32),
        Float(f32),
        Identifier(String),
        ParenExpr(Box<Expression>),
        BinaryExpr {
            operator: BinOpr,
            lhs: Box<Expression>,
            rhs: Box<Expression>,
        },
        UnaryExpr {
            operator: UnOpr,
            operand: Box<Expression>,
        },
        FieldAccess(String, Box<Expression>),
        ArrayAccess(String, Box<Expression>),
    }

    #[derive(Debug, Clone)]
    enum Lvalue {
        Variable(String),
        FieldAccess(String, Box<Lvalue>),
        ArrayAccess(Box<Lvalue>, Box<Expression>),
    }

    #[derive(Debug, Clone)]
    enum Type {
        Int,
        Float,
        Boolean,
        String,
        Nil,
        User(String),
    }

    #[derive(Debug, Clone)]
    enum Statement {
        Assignment(Lvalue, Box<Expression>),
        WhileLoop(Box<Expression>, Vec<Statement>),
        IfStatement {
            condition: Box<Expression>,
            thenBlock: Vec<Statement>,
            elseBlock: Option<Vec<Statement>>,
        },
        Function {
            name: String,
            args: Vec<String>,
            body: Vec<Statement>,
        },
        ReturnStatement(Option<Expression>),
        FunctionCall(String, Vec<Expression>),
    }

    use chumsky::prelude::*;

    #[test]
    fn chumsky_tutorial() {
        let input = r#"x = 10"#;
        let mut parser = parser();
        let result = parser.parse(input);
        println!("{:?}", result);
    }

    fn parser() -> impl Parser<char, Statement, Error = Simple<char>> {
        let integer = text::int(10)
            .map(|num: String| Expression::Integer(num.parse::<i32>().unwrap()))
            .padded();

        let identifier = text::ident().padded();

        let float = integer.then(just('.').then(integer)).or_not().padded();

        let atoms = |c| just(c).padded();

        let lparen = atoms('(');
        let rparen = atoms(')');
        let lbrace = atoms('{');
        let rbrace = atoms('}');
        let lbracket = atoms('[');
        let rbracket = atoms(']');
        let plus = atoms('+');
        let rem = atoms('%');
        let minus = atoms('-');
        let mult = atoms('*');
        let div = atoms('/');
        let assignequal = atoms('=');
        let not = atoms('!');
        let lt = atoms('<');
        let gt = atoms('>');
        let eq = atoms('=');
        let neq = atoms('!');

        let variable = identifier;
        let field_access = identifier.then(just('.').then(identifier));

        let expr =
            recursive(|expr| {
                let addexpr = integer.then(plus.then(integer)).map(|(lhs, (_, rhs))| {
                    Expression::BinaryExpr {
                        operator: BinOpr::Add,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                });

                let subexpr = integer.then(minus.then(integer)).map(|(lhs, (_, rhs))| {
                    Expression::BinaryExpr {
                        operator: BinOpr::Sub,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                });

                let mulexpr = integer.then(mult.then(integer)).map(|(lhs, (_, rhs))| {
                    Expression::BinaryExpr {
                        operator: BinOpr::Mul,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                });

                let divexpr =
                    integer
                        .then(div.then(integer))
                        .map(|(lhs, (_, rhs))| Expression::BinaryExpr {
                            operator: BinOpr::Div,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });

                let remexpr =
                    integer
                        .then(rem.then(integer))
                        .map(|(lhs, (_, rhs))| Expression::BinaryExpr {
                            operator: BinOpr::Rem,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });

                // Return some kind of integer/float.
                let arith_exprs = addexpr.or(subexpr).or(mulexpr).or(divexpr).or(remexpr);

                let boolean_cmp_less_expr =
                    integer
                        .then(lt.then(integer))
                        .map(|(lhs, (_, rhs))| Expression::BinaryExpr {
                            operator: BinOpr::BooleanCmpLess,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });

                let boolean_cmp_greater_expr =
                    integer
                        .then(gt.then(integer))
                        .map(|(lhs, (_, rhs))| Expression::BinaryExpr {
                            operator: BinOpr::BooleanCmpGreater,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        });

                // Return some kind of boolean value.
                let boolean_exprs = boolean_cmp_less_expr.or(boolean_cmp_greater_expr);

                // Returns an int/float but is unary.
                let unary_expr = not.or(minus).then(expr).map(|(op, expr)| match op {
                    '-' => Expression::UnaryExpr {
                        operator: UnOpr::Neg,
                        operand: Box::new(expr),
                    },
                    '!' => Expression::UnaryExpr {
                        operator: UnOpr::Not,
                        operand: Box::new(expr),
                    },
                    _ => unreachable!(),
                });

                // Return an expression.
                let exprs = arith_exprs.or(boolean_exprs).or(unary_expr).or(integer);

                exprs
            });

        let array_access = identifier.then(just('[').then(expr.clone()).then(just(']')));

        let statement = recursive(|statement| {
            let variable_assignment =
                variable
                    .then(assignequal)
                    .then(expr.clone())
                    .map(|((var, _), expr)| {
                        Statement::Assignment(Lvalue::Variable(var), Box::new(expr.clone()))
                    });

            variable_assignment
        });

        statement
    }
}
