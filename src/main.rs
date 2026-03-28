mod lex;
mod parse;
mod types;

use std::{
    env,
    fs::File,
    io::{
        BufReader,
        Read
    },
    error::Error,
};

use crate::{lex::{Lexer, Token, TokenType}, parse::{Node, Parser}};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Kernlang Compiler v0.0.1");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: kern <source-file>");
        return Ok(());
    }

    // Tokenization
    let mut code = std::fs::read_to_string(&args[1])?;
    code += "\0";
    let buffer = code.as_bytes();
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

    println!("Number of tokens: {:?}", tokstream.len());

    // Print tokens for debugging
    for iter in tokstream.iter() {
        println!("{:?}", iter);
        println!("{}", &code[iter.tok_pos..iter.tok_pos+iter.tok_len]);
    }

    // Parsing
    let mut parser = Parser::new(&buffer, &tokstream);
    let mut nodestream : Vec<Node> = Vec::new();
    
    for _token in tokstream.iter(){
        let node = parser.create_node();
        match node {
            Node::LitInt(_) => nodestream.push(node),
            _ => {}
        }
    }

    println!("Number of nodes: {:?}", nodestream.len());

    for iter in nodestream.iter() {
        println!("Expression: {:?}", iter);
    }

    return Ok(());
}