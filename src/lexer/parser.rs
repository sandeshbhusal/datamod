use super::lexer::Lexer;
use super::lexer::Token;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

#[derive(Debug)]
pub enum JsonType {
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
        if let Some(token) = self.tokens.next() {
            match token {
                Token::STRING(s) => JsonType::String(s.clone()),
                Token::FLOAT(n) => JsonType::Float(*n),
                Token::INTEGER(n) => JsonType::Integer(*n),
                Token::BOOLEAN(b) => JsonType::Boolean(*b),
                Token::NONE => JsonType::Null,
                Token::LBRACE => self.parse_object(),
                Token::LBRAC => self.parse_array(),
                _ => panic!("Unexpected token: {:?}; remaining tokens {:?}", token, self.tokens),
            }
        } else {
            panic!("Unexpected end of tokens")
        }
    }

    fn parse_object(&mut self) -> JsonType {
        let mut object = vec![];

        loop {
            // If the next token is a closing brace '}', return the object.
            match self.tokens.peek() {
                Some(&Token::RBRACE) => {
                    // Consume the closing brace.
                    self.tokens.next();
                    break;
                }
                Some(&Token::STRING(_)) => {
                    // Parse the key.
                    let key_token = self.tokens.next().unwrap();
                    let key: String = if let Token::STRING(key) = key_token {
                        key.clone()
                    } else {
                        panic!("Expected a string key, got {:?}", key_token);
                    };

                    // Expect a colon.
                    match self.tokens.next() {
                        Some(Token::COLON) => {} // Consume the colon.
                        other => panic!("Expected a colon after key, got {:?}", other),
                    }

                    // Parse the value.
                    let value = self.parse_value();

                    // Add the key-value pair to the object.
                    object.push((key, value));

                    // Check the next token to decide whether to continue or end the object.
                    match self.tokens.peek() {
                        Some(&Token::COMMA) => {
                            // Consume the comma and continue parsing the next key-value pair.
                            self.tokens.next();
                        }
                        Some(&Token::RBRACE) => {
                            // We are at the end of the object. Consume the closing brace.
                            self.tokens.next();
                            break;
                        }
                        other => {
                            panic!("Expected a comma or closing brace, got {:?}", other);
                        }
                    }
                }
                _ => {
                    panic!(
                        "Expected a string key or closing brace, got {:?}",
                        self.tokens.peek()
                    );
                }
            }
        }

        JsonType::Object(object)
    }

    fn parse_array(&mut self) -> JsonType {
        // parse items until we see a closing bracket.
        // return an array.
        let mut rval = vec![];
        while self.tokens.peek() != Some(&&Token::RBRAC) {
            rval.push(self.parse_value());
            println!("rval: {:?}", rval);

            // Next token could either be a comma, or a closing bracket.
            match self.tokens.peek() {
                Some(&Token::COMMA) => {
                    // Consume the comma.
                    self.tokens.next();
                }
                Some(&Token::RBRAC) => {
                    // We are at the end of the array. Consume the closing bracket.
                    self.tokens.next();
                    break;
                }
                other => {
                    panic!("Expected a comma or closing bracket, got {:?}", other);
                }
            }
        }

        JsonType::Array(rval)
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
