use axi_cas::{lexer::Lexer, Token};

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut lexer = Lexer::new(input.as_str());
    loop {
        let next = lexer.next();
        println!("{next:?}: {next}");
        if next == Token::EOF {
            break;
        }
    }
}