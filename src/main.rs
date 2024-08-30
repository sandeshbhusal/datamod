mod lexer;

fn main() {
    // An empty object is a valid json.
    let mut lex = lexer::lexer::Lexer::new("{}");
    let parsed_ast = lex.parse(); 
    println!("{:?}", parsed_ast);
}
