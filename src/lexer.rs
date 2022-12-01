use std::str::Bytes;

enum TokenType {
    LCBracket, // {
    RCBracket, // }
    Add,       // +
    Sub,       // -
    Inc,       // +
}

pub struct Token {
    t_type: TokenType,
    val: String,
}

pub fn lex(mut inp: Bytes) -> Vec<Token> {
    let out: Vec<Token> = Vec::new();
    let mut idx: i32 = 0;

    while idx < inp.len() as i32 {
        let c: char = inp.nth(idx as usize).unwrap() as char;

        match c {
            _ => {
                println!("Unknown token: {}", c);
            }
        }

        idx += 1;
    }

    return out;
}