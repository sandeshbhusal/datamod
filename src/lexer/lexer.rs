#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i64),
}

pub struct LexResult<'a> {
    pub(crate) remaining: &'a str,
    pub(crate) token: Option<Token>,
}

impl<'a> LexResult<'a> {
    pub fn new(remaining: &'a str, token: Option<Token>) -> Self {
        Self { remaining, token }
    }

    pub fn then(&self, lexer: &mut (dyn Lexer<'a> + 'a)) -> LexResult<'a> {
        lexer.set_input(self.remaining);
        lexer.next()
    }
}

pub trait Lexer<'a> {
    fn next(&mut self) -> LexResult<'a>;
    fn set_input(&mut self, input: &'a str);
}


pub struct Fail {
    reason: &'static str,
}

impl Fail {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}

impl<'a> Lexer<'a> for Fail {
    fn next(&mut self) -> LexResult<'a> {
        panic!("failed with reason : {}", self.reason)
    }

    fn set_input(&mut self, _input: &'a str) {
        unreachable!("Unreachable for fail lexer.")
    }
}
