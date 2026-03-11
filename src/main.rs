mod lex;

use std::{
    env,
    fs::File,
    io::{
        BufReader,
        Read
    },
    error::Error,
};

use crate::lex::{Lexer, Token, TokenType};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Kernlang Compiler v0.0.1");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: kern <source-file>");
        return Ok(());
    }

    let mut code = std::fs::read_to_string(&args[1])?;
    code += "\0";
    let buffer = code.to_owned();
    let mut lexer = Lexer::new(&buffer);
    lexer.length += 1;
    let mut tokstream : Vec<Token> = Vec::new();

    loop {
        let token = lexer.create_token();
        match token.token_type{
            TokenType::EOF => break,
            TokenType::Error => {},
            _ => tokstream.push(token),
        }
    }

    for iter in tokstream.iter() {
        println!("{:?}", iter);
        println!("{}", &code[iter.tok_pos..iter.tok_pos+iter.tok_len]);
    }
    return Ok(());
}