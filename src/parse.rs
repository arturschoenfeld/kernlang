use std::i128;
use crate::lex::{LiteralType, Token, TokenType};
use crate::types::int;

#[derive(Debug)]
pub enum Operator{
    Asg,
    Plus, Minus, Mul, Div, Mod,
    Dot, Arrow, FatArrow, Range, RangeIncl,
    Wildcard,
    Quantifier,
    Pointer, Address,
    BrackL, BrackR, ParenL, ParenR,
    And, Or, Not,
    Xor, Shl, Shr,
    Eq, Neq, Gt, Gte, Lt, Lte
}

#[derive(Debug)]
pub enum Node{
	LitInt(i128),
    LitReal(f64),
    UnaryExpr{
        op: Operator,
        child: Box<Node>
    },
    BinaryExpr{
        op: Operator,
        left: Box<Node>,
        right: Box<Node>
    },
    Wip,
    Error
}

#[derive(Debug)]
pub struct Parser<'src, 'tok>{
    pub src: &'src [u8],
    pub tok_stream: &'tok Vec<Token>,
    pub elem_cnt: usize,
    pub pos: usize
}

impl<'src, 'tok> Parser<'src, 'tok>{
    pub fn new(source: &'src [u8], stream: &'tok Vec<Token>) -> Self{
        Parser{
            src: source,
            tok_stream: stream,
            elem_cnt: stream.len(),
            pos: 0
        }
    }

    #[inline(always)]
    fn peek(&self) -> *const Token{
        &self.tok_stream[self.pos]
    }

    #[inline(always)]
    fn advance(&mut self){
        if self.peek().is_null(){
            return;
        }
        self.pos += 1;
    }

    /// Similarly to the `create_token` logic, the node dispatch depends on the first token
    /// of the new expression or statement.
    #[inline(always)]
    pub fn create_node(&mut self) -> Node{
        let start = self.pos;
        
        unsafe{
            let tok = std::ptr::read(self.peek()); 
            let node = match tok.token_type {
                TokenType::Ident => {
                    println!("Identifiers are not supported yet.");
                    Node::Wip
                },
                TokenType::Keyword(_) => {
                    println!("Keywords are not supported yet.");
                    Node::Wip
                },
                TokenType::Literal(lit) => match lit {
                    LiteralType::Int => {
                        let lit_int = &self.src[tok.tok_pos..tok.tok_pos+tok.tok_len];
                        Node::LitInt(
                            int::parse_int(lit_int).unwrap()
                        )
                    }
                    LiteralType::Real => {
                        println!("Real numbers are not supported yet.");
                        Node::Wip
                    },
                    LiteralType::Char => {
                        println!("Characters are not supported yet.");
                        Node::Wip
                    },
                    LiteralType::String => {
                        println!("Strings are not supported yet.");
                        Node::Wip
                    },
                },
                TokenType::Punc(_) => {
                    println!("Punctuation is not supported yet.");
                    Node::Wip
                },
                TokenType::Error => {
                    panic!("Erroneous token found. Abort.");
                },
                _ => Node::Error,
            };  
            self.advance();
            node
        }
    }
}