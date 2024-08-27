mod tutorial {
    use chumsky::prelude::*;

    use crate::lparser::Expr;

    #[test]
    fn chumsky_tutorial() {
        let src = r#" 123+ 2"#;

        println!("{:?}", parser().parse(src).unwrap().eval());
    }

    fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
        let int = text::int(10)
            .map(|s: String| Expr::Number(s.parse().unwrap()))
            .padded();

        let atom = int;
        let op = |c| just(c).padded();
        let unary = op('-').then(atom);

        let sumexpr = (atom
            .clone()
            .then(op('+').then(atom))
            .map(|(left, (_, right))| Expr::AddExpr(Box::new(left), Box::new(right))));

        let subexpr = (atom
            .clone()
            .then(op('-').then(atom))
            .map(|(left, (_, right))| Expr::SubExpr(Box::new(left), Box::new(right))));

        let mulexpr = (atom
            .clone()
            .then(op('*').then(atom))
            .map(|(left, (_, right))| Expr::MulExpr(Box::new(left), Box::new(right))));

        let divexpr = (atom
            .clone()
            .then(op('/').then(atom))
            .map(|(left, (_, right))| Expr::DivExpr(Box::new(left), Box::new(right))));

        let expr = sumexpr.or(subexpr).or(mulexpr).or(divexpr);

        expr.then_ignore(end())
    }
}
