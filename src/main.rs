use std::str::Bytes;

enum TokenType {
    SET
}

struct Token {
    t_type: TokenType,
    val: String,
}

fn lex(inp: Bytes) -> Vec<Token> {
    let out: Vec<Token> = Vec::new();
    let idx: i32 = 0;

    while idx < inp.len() {
        
    }

    return out;
}

fn main() {
    println!("Hello, world!");
}
