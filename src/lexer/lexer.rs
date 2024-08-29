#![allow(dead_code)]

struct Lexer<'a> {
    input: &'a str,
}

#[derive(Debug)]
pub enum Token {
    LBRACE,
    RBRACE,
    LBRAC,
    RBRAC,
    COLON,
    COMMA,
    QUOTE,

    STRING(String),
    INTEGER(i64),
    FLOAT(f64),
    BOOLEAN(bool),
    NONE,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn parse(&mut self) -> Vec<Token> {
        let mut iter = self.input.chars().into_iter().peekable();
        let mut tokens = Vec::new();
        let mut linum = 1;
        let mut offset = 0;

        while let Some(ch) = iter.peek() {
            if *ch == 0x0A as char {
                linum += 1;
            }

            // Eat whitespace
            if ch.is_whitespace() {
                iter.next();
                continue;
            }

            match ch {
                '{' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::LBRACE);
                }
                '}' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::RBRACE);
                }
                '[' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::LBRAC);
                }
                ']' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::RBRAC);
                }
                ':' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::COLON);
                }
                ',' => {
                    offset += 1;
                    iter.next();
                    tokens.push(Token::COMMA);
                }
                '"' => {
                    iter.next();
                    let mut string = String::new();
                    let mut endfound = false;
                    while let Some(ch) = iter.next() {
                        if ch == '"' {
                            endfound = true;
                            break;
                        }
                        string.push(ch);
                    }
                    if !endfound {
                        panic!("Unterminated string found");
                    }
                    offset += string.len() + 2;
                    tokens.push(Token::STRING(string));
                }
                '0'..='9' => {
                    let mut number = String::new();
                    while let Some(ch) = iter.peek() {
                        if ch.is_ascii_digit() {
                            number.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    // Check if it's immediately followed by a dot, if so, it's a float.
                    let mut floatval = false;
                    if let Some(ch) = iter.peek() {
                        if *ch == '.' {
                            floatval = true;
                            number.push(iter.next().unwrap());
                            while let Some(ch) = iter.peek() {
                                if ch.is_ascii_digit() {
                                    number.push(iter.next().unwrap());
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    if floatval {
                        match number.parse() {
                            Ok(n) => tokens.push(Token::FLOAT(n)),
                            Err(_) => panic!("Invalid number found"),
                        }
                    } else {
                        match number.parse() {
                            Ok(n) => tokens.push(Token::INTEGER(n)),
                            Err(_) => panic!("Invalid number found"),
                        }
                    };

                    offset += number.len();
                }
                _ => {
                    // Try to parse a string.
                    let mut string = String::new();
                    while let Some(ch) = iter.peek() { 
                        if ch.is_alphabetic() {
                            string.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    // Match true/false.
                    match string.as_str() {
                        "true" => tokens.push(Token::BOOLEAN(true)),
                        "false" => tokens.push(Token::BOOLEAN(false)),
                        "null" => tokens.push(Token::NONE),
                        _ => panic!("Invalid stray string found: {}", string),
                    }
                }
            }
        }
        tokens
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
    println!("{:?}", tokens);
}
