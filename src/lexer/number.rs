use super::lexer::{LexResult, Lexer, Token};


pub struct ConsumeNumber<'a> {
    inputstream: &'a str,
}

impl<'a> ConsumeNumber<'a> {
    pub fn new(inputstream: &'a str) -> Self {
        Self { inputstream }
    }
}

impl<'a> Lexer<'a> for ConsumeNumber<'a> {
     fn next(&mut self) -> LexResult<'a> {
        if self.inputstream.starts_with(|c: char| c.is_ascii_digit()) {
            let end_pos = self.inputstream.find(|c: char| !c.is_ascii_digit()).unwrap_or_else(|| self.inputstream.len());
            let (number, remaining) = self.inputstream.split_at(end_pos);

            let token = match number.parse() {
                Ok(n) => Some(Token::Int(n)),
                Err(_) => None,
            };

            LexResult::new(remaining, token)
        } else {
            // Nothing was parsed by this.
            LexResult::new(self.inputstream, None)
        }
    }

     fn set_input(&mut self, input: &'a str) {
        self.inputstream = input;
    }
}


#[test]
fn consume_test() {
    let mut lexer = ConsumeNumber::new("123 123");
    let result = lexer.next().then(ConsumeNumber {inputstream: ""});
    assert_eq!(result.token, Some(Token::Int(123)));
    assert_eq!(result.remaining, "abc");
}
