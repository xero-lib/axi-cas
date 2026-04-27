use axi_cas::{lexer::Lexer, Token};

fn main() {
    let mut lexer = Lexer::new("1 + 2 + 3^4 * 10");
    loop {
        let next = lexer.next();
        println!("{next}");
        if next == Token::EOF {
            break;
        }
    }
}