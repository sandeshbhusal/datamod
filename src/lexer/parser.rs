use super::lexer::Lexer;
use super::lexer::Token;
use std::iter::Peekable;

#[derive(Debug)]
struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

#[derive(Debug)]
enum JsonType {
    String(String),
    Boolean(bool),
    Float(f64),
    Integer(i64),
    Array(Vec<JsonType>),
    Object(Vec<(String, JsonType)>),
    Null,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    fn parse_value(&mut self) -> JsonType {
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::STRING(s) => JsonType::String(s.clone()),
                Token::FLOAT(n) => JsonType::Float(*n),
                Token::INTEGER(n) => JsonType::Integer(*n),
                Token::BOOLEAN(b) => JsonType::Boolean(*b),
                Token::NONE => JsonType::Null,
                Token::LBRACE => self.parse_object(),
                Token::LBRAC => self.parse_array(),
                _ => panic!("Unexpected token: {:?}", token),
            }
        } else {
            panic!("Unexpected end of tokens")
        }
    }

    fn parse_object(&mut self) -> JsonType {
        // Eat the brace token
        self.tokens.next();
        // Eat a string as the key.
        let key = match self.tokens.next() {
            Some(Token::STRING(s)) => s.clone(),
            _ => panic!("Expected string key"),
        };

        // Eat the colon token
        self.tokens.next();

        // Eat the value, parse with self.parse_value
        let value = self.parse_value();

        // Eat the closing brace token.
        self.tokens.next();

        // emit object.
        JsonType::Object(vec![(key, value)])
    }

    fn parse_array(&mut self) -> JsonType {
        unimplemented!()
    }

    fn ast(&mut self) -> JsonType {
        self.parse_value()
    }
}

#[test]
fn lexer_test() {
    let teststr = r#"
    {
        "string": "Hello, World!",
        "number": 42,
        "float": 3.14,
        "boolean_true": true,
        "boolean_false": false,
        "null_value": null,
        "array": [
          "item1",
          2,
          false,
          null,
          {
            "nested_object": {
              "key1": "value1",
              "key2": 100
            }
          }
        ],
        "object": {
          "nested_string": "Nested Hello",
          "nested_number": 99,
          "nested_array": [
            "array_item1",
            {
              "deeply_nested_object": {
                "deep_key": "deep_value",
                "deep_number": 7
              }
            }
          ]
        }
      }
      "#;
    let mut lexer = Lexer::new(&teststr);
    let tokens = lexer.parse();

    let mut parser = Parser::new(&tokens);
    let ast = parser.ast();

    println!("{:?}", ast);
}
