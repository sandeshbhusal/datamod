use std::io::Read;

mod lexer;

fn main() {
    // Read first argument for filename,
    // open the file, and parse it.
    let filename = std::env::args().nth(1).expect("Usage: datamod <filename>");
    println!("filename: {}", filename);

    let mut file = std::fs::File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut lexer = lexer::Lexer::new(&contents);
    let tokens = lexer.parse();

    // println!("tokens: {:?}", tokens);
}
