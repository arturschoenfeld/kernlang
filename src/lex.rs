#[derive(Debug, Default)]
pub struct Token{
    pub token_type : TokenType,
    pub src_line : usize,
    pub src_col : usize,
    pub tok_pos : usize,
    pub tok_len : usize
}

#[derive(Debug, Default)]
pub enum TokenType{
    EOF,
    Ident,
    Keyword(Keyword),
    Literal(LiteralType),
    Punc(Punc),
    Attribute,              // unused
    #[default]
    Error
}

#[derive(Debug)]
pub enum Keyword{
    // Data Types
    ISIZE, I8, I16, I32, I64, I128,
    USIZE, U8, U16, U32, U64, U128,
    FIXSIZE, FIX8, FIX16, FIX32, FIX64, FIX128,
    FLT16, FLT32, FLT64, FLT128, BFLT16,
    BOOL,

    // Literals
    NULL,
    TRUE, FALSE,

    // Control FLow
    IF, ELSE, MATCH,
    LOOP, FOR, WHILE,
    BREAK, SKIP,
    RETURN,
    DEFER,

    // Contracts
    PRE, INV, POST,

    // Aggregate Data
    STRUCT, UNION, ENUM, SUM,

    // Top-Level Definitions
    VAR, FN, USE,

    // Intermediate Clauses
    AS, TO, IN, OF,

    // Qualifiers
    CONST, PUB, VOL, PACK,

    // Testing
    TEST, ASSERT,

    // Coroutines
    COROUT, RUN, YIELD, CANCEL,

    // Interop
    ASM, EXPORT, EXTERN,

    // Casts
    BITCAST, VALCAST,

    // Type Introspection and Alignment
    ALIGN, SIZE, TYPE,

    // Bitwise and Boolean Operators
    AND, OR, NOT,
    XOR, SHL, SHR,
    EQ, NEQ, GT, GTE, LT, LTE
}

#[derive(Debug)]
pub enum Punc{
    // Non-Alphabetical Operators
    Assignment,                             // =
    Plus, Minus, Mul, Div, Mod,             // + - * / %
    Dot, Arrow, FatArrow, Range, RangeIncl, // . -> => .. ..=
    Wildcard,                               // _
    Quantifier,                             // :
    Pointer, Address,                       // ^ @
    BrackL, BrackR, ParenL, ParenR,          // [] ()

    // Syntax-Related Punctuators
    BraceL, BraceR,         // {}
    Comma, Semicolon,       // , ;
    GenL, GenR,             // <>
    Doublequote, Dollar,    // " $
    Docs,                    // ##, not relevant for actual codegen
    Comment,
    Unknown
}

#[derive(Debug)]
pub enum LiteralType{
    Int,
    Real,
    Char,
    String
}

#[derive(Debug)]
pub struct Lexer<'src>{
    source: &'src[u8],
    pub length: usize,
    pub pos: usize,
    line: usize,
    col: usize
}

static CHAR_CLASS: [u8; 128] = {
    let mut table = [0; 128];
    let mut i  = 0;
    while i < 128 {
        table[i] = match i as u8{
            b'a' ..= b'z' | b'A' ..= b'Z' => 1,
            b'0' ..= b'9' => 2,
            b'=' | b'+' | b'-' | b'*' | b'/' | b'%' | b'.' | b'_' | b':' | b'^' | b'@' |
            b'[' | b']' | b'(' | b')' | b'{' | b'}' | b',' | b';' | b'<' | b'>' | b'"' |
            b'#' | b'$' => 3,
            _ => 0
        };
        i += 1;
    }
    table
};

impl<'src> Lexer<'src>{
    pub fn new(source: &'src str) -> Self{
        let mut bytes = Vec::with_capacity(source.len() + 1);
        bytes.extend_from_slice((source.as_bytes()));
        bytes.push(0);
        let slice = Box::leak(bytes.into_boxed_slice());
        Lexer{
            source: slice,
            length: source.len() + 1,
            pos: 0,
            line: 1,
            col: 1
        }
    }

    fn peek(&self) -> *const u8{
        &self.source[self.pos]
    }

    fn advance(&mut self){
        unsafe{
            match *self.peek(){
                0 => return,
                b'\n' => {
                    self.line += 1;
                    self.col = 1;
                },
                _ => self.col += 1,
            }
        }
        self.pos += 1;
    }

    fn skip_nontokens(&mut self) {
        loop {
            unsafe {
                match *self.peek() {
                    b' ' | b'\t' | b'\r' => self.advance(),
                    _ => break,
                }
            }
        }
    }

    pub fn create_token(&mut self) -> Token {
        self.skip_nontokens();
        let start = self.pos;

        unsafe{
            match CHAR_CLASS[*self.peek() as usize]{
                1 => {
                    return Token { 
                        token_type : self.detect_id_or_kwrd(),
                        src_line : self.line,
                        src_col : self.col,
                        tok_pos : start,
                        tok_len : self.pos - start
                    }       
                },
                2 => {
                    return Token { 
                        token_type : TokenType::Literal(self.detect_number()),
                        src_line : self.line,
                        src_col : self.col,
                        tok_pos : start,
                        tok_len : self.pos - start
                    }
                },
                3 => {
                    return Token { 
                        token_type : TokenType::Punc(self.detect_punc()),
                        src_line : self.line,
                        src_col : self.col,
                        tok_pos : start,
                        tok_len : self.pos - start
                    }
                },
                _ => {
                    self.advance();
                    if *self.peek() == 0{
                        return Token{
                            token_type : TokenType::EOF,
                            src_line : self.line,
                            src_col : self.col,
                            tok_pos : start,
                            tok_len : self.pos - start
                        }
                    } else {
                        return Token{
                            token_type : TokenType::Error,
                            src_line : self.line,
                            src_col : self.col,
                            tok_pos : start,
                            tok_len : self.pos - start
                        }
                    }
                }
            }
        }
    }

    fn detect_id_or_kwrd(&mut self) -> TokenType{
        let start = self.pos;
        unsafe{
            while (CHAR_CLASS[*self.peek() as usize] == 1)
            || (CHAR_CLASS[*self.peek() as usize] == 2)
            || (*self.peek() == b'_'){
                self.advance();
            }
        }

        if (self.pos - start) > 7 {
            return TokenType::Ident;
        }

        match &self.source[start..self.pos] as &[u8] {
            b"isize" => return TokenType::Keyword(Keyword::ISIZE),
            b"i8" => return TokenType::Keyword(Keyword::I8),
            b"i16" => return TokenType::Keyword(Keyword::I16),
            b"i32" => return TokenType::Keyword(Keyword::I32),
            b"i64" => return TokenType::Keyword(Keyword::I64),
            b"i128" => return TokenType::Keyword(Keyword::I128),
            b"usize" => return TokenType::Keyword(Keyword::USIZE),
            b"u8" => return TokenType::Keyword(Keyword::U8),
            b"u16" => return TokenType::Keyword(Keyword::U16),
            b"u32" => return TokenType::Keyword(Keyword::U32),
            b"u64" => return TokenType::Keyword(Keyword::U64),
            b"u128" => return TokenType::Keyword(Keyword::U128),
            b"fixsize" => return TokenType::Keyword(Keyword::FIXSIZE),
            b"fix8" => return TokenType::Keyword(Keyword::FIX8),
            b"fix16" => return TokenType::Keyword(Keyword::FIX16),
            b"fix32" => return TokenType::Keyword(Keyword::FIX32),
            b"fix64" => return TokenType::Keyword(Keyword::FIX64),
            b"fix128" => return TokenType::Keyword(Keyword::FIX128),
            b"flt16" => return TokenType::Keyword(Keyword::FLT16),
            b"flt32" => return TokenType::Keyword(Keyword::FLT32),
            b"flt64" => return TokenType::Keyword(Keyword::FLT64),
            b"flt128" => return TokenType::Keyword(Keyword::FLT128),
            b"bflt16" => return TokenType::Keyword(Keyword::BFLT16),
            b"bool" => return TokenType::Keyword(Keyword::BOOL),
            b"null" => return TokenType::Keyword(Keyword::NULL),
            b"true" => return TokenType::Keyword(Keyword::TRUE),
            b"false" => return TokenType::Keyword(Keyword::FALSE),
            b"if" => return TokenType::Keyword(Keyword::IF),
            b"else" => return TokenType::Keyword(Keyword::ELSE),
            b"match" => return TokenType::Keyword(Keyword::MATCH),
            b"loop" => return TokenType::Keyword(Keyword::LOOP),
            b"for" => return TokenType::Keyword(Keyword::FOR),
            b"while" => return TokenType::Keyword(Keyword::WHILE),
            b"break" => return TokenType::Keyword(Keyword::BREAK),
            b"skip" => return TokenType::Keyword(Keyword::SKIP),
            b"return" => return TokenType::Keyword(Keyword::RETURN),
            b"defer" => return TokenType::Keyword(Keyword::DEFER),
            b"pre" => return TokenType::Keyword(Keyword::PRE),
            b"inv" => return TokenType::Keyword(Keyword::INV),
            b"post" => return TokenType::Keyword(Keyword::POST),
            b"struct" => return TokenType::Keyword(Keyword::STRUCT),
            b"union" => return TokenType::Keyword(Keyword::UNION),
            b"enum" => return TokenType::Keyword(Keyword::ENUM),
            b"sum" => return TokenType::Keyword(Keyword::SUM),
            b"var" => return TokenType::Keyword(Keyword::VAR),
            b"fn" => return TokenType::Keyword(Keyword::FN),
            b"use" => return TokenType::Keyword(Keyword::USE),
            b"as" => return TokenType::Keyword(Keyword::AS),
            b"to" => return TokenType::Keyword(Keyword::TO),
            b"in" => return TokenType::Keyword(Keyword::IN),
            b"of" => return TokenType::Keyword(Keyword::OF),
            b"const" => return TokenType::Keyword(Keyword::CONST),
            b"pub" => return TokenType::Keyword(Keyword::PUB),
            b"vol" => return TokenType::Keyword(Keyword::VOL),
            b"pack" => return TokenType::Keyword(Keyword::PACK),
            b"test" => return TokenType::Keyword(Keyword::TEST),
            b"assert" => return TokenType::Keyword(Keyword::ASSERT),
            b"corout" => return TokenType::Keyword(Keyword::COROUT),
            b"run" => return TokenType::Keyword(Keyword::RUN),
            b"yield" => return TokenType::Keyword(Keyword::YIELD),
            b"cancel" => return TokenType::Keyword(Keyword::CANCEL),
            b"asm" => return TokenType::Keyword(Keyword::ASM),
            b"export" => return TokenType::Keyword(Keyword::EXPORT),
            b"extern" => return TokenType::Keyword(Keyword::EXTERN),
            b"bitcast" => return TokenType::Keyword(Keyword::BITCAST),
            b"valcast" => return TokenType::Keyword(Keyword::VALCAST),
            b"align" => return TokenType::Keyword(Keyword::ALIGN),
            b"size" => return TokenType::Keyword(Keyword::SIZE),
            b"type" => return TokenType::Keyword(Keyword::TYPE),
            b"and" => return TokenType::Keyword(Keyword::AND),
            b"or" => return TokenType::Keyword(Keyword::OR),
            b"not" => return TokenType::Keyword(Keyword::NOT),
            b"xor" => return TokenType::Keyword(Keyword::XOR),
            b"shl" => return TokenType::Keyword(Keyword::SHL),
            b"shr" => return TokenType::Keyword(Keyword::SHR),
            b"eq" => return TokenType::Keyword(Keyword::EQ),
            b"neq" => return TokenType::Keyword(Keyword::NEQ),
            b"gt" => return TokenType::Keyword(Keyword::GT),
            b"gte" => return TokenType::Keyword(Keyword::GTE),
            b"lt" => return TokenType::Keyword(Keyword::LT),
            b"lte" => return TokenType::Keyword(Keyword::LTE),
            _ => return TokenType::Ident
        }
    }

    fn detect_number(&mut self) -> LiteralType{
        enum State{
            Start, Startzero,
            Dec, DecDot, DecFrac, DecExp, DecExpSign, DecExpDig,
            Bin, Oct,
            Hex, HexDot, HexFrac, HexExp, HexExpSign, HexExpDig
        }

        let mut state = State::Start;

        loop{
            unsafe {
                match state {

                    State::Start => {
                        if *self.peek() == b'0' {
                            state = State::Startzero;
                        } else if CHAR_CLASS[*self.peek() as usize] == 2 {
                            state = State::Dec;
                        } else {
                            return LiteralType::Int;
                        }
                    },

                    State::Startzero => {
                        match *self.peek(){
                            b'b' | b'B' => state = State::Bin,
                            b'o' | b'O' => state = State::Oct,
                            b'x' | b'X' => state = State::Hex,
                            b'.' => state = State::DecDot,
                            b'e' | b'E' => state = State::DecExp,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    state = State::Dec;
                                } else {
                                    return LiteralType::Int;
                                }
                            }
                        }
                    },

                    State::Dec => {
                        match *self.peek(){
                            b'.' => state = State::DecDot,
                            b'e' | b'E' => state = State::DecExp,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    {};
                                } else {
                                    return LiteralType::Int;
                                }                                
                            }
                        }
                    },
                    
                    State::DecDot => {
                        if CHAR_CLASS[*self.peek() as usize] == 2 {
                            state = State::DecFrac;
                        } else {
                            return LiteralType::Real;
                        }
                    },

                    State::DecFrac => {
                        match *self.peek(){
                            b'e' | b'E' => state = State::DecExp,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    {};
                                } else {
                                    return LiteralType::Real;
                                }
                            }
                        }
                    },

                    State::DecExp => {
                        match *self.peek(){
                            b'+' | b'-' => state = State::DecExpSign,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    state = State::DecExpDig;
                                } else {
                                    return LiteralType::Real;
                                }
                            }
                        }
                    },

                    State::DecExpSign => {
                        if CHAR_CLASS[*self.peek() as usize] == 2 {
                            state = State::DecExpDig;
                        } else {
                            return LiteralType::Real;
                        }
                    },

                    State::DecExpDig => {
                        if CHAR_CLASS[*self.peek() as usize] == 2 {
                            {};
                        } else {
                            return LiteralType::Real;
                        }
                    },

                    State::Bin => {
                        if (*self.peek() != b'0') && (*self.peek() != b'1'){
                            return LiteralType::Int;
                        }
                    },

                    State::Oct => {
                        if (CHAR_CLASS[*self.peek() as usize] == 2) &&
                        (*self.peek() != b'8') && (*self.peek() != b'9'){
                            {};
                        } else {
                            return LiteralType::Int;
                        }
                    },

                    State::Hex => {
                        match *self.peek(){
                            b'p' | b'P' => state = State::HexExp,
                            b'.' => state = State::HexDot,
                            b'a' ..= b'f' | b'A' ..= b'F' => {},
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    {};
                                } else {
                                    return LiteralType::Int;
                                }
                            },
                        }
                    },

                    State::HexDot => {
                        match *self.peek(){
                            b'a' ..= b'f' | b'A' ..= b'F' => state = State::HexFrac,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    state = State::HexFrac;
                                } else {
                                    return LiteralType::Real;
                                }
                            },
                        }
                    },

                    State::HexFrac => {
                         match *self.peek(){
                            b'p' | b'P' => state = State::HexExp,
                            b'a' ..= b'f' | b'A' ..= b'F' => {},
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    {};
                                } else {
                                    return LiteralType::Real;
                                }
                            },
                        }
                    },

                    State::HexExp => {
                        match *self.peek(){
                            b'+' | b'-' => state = State::HexExpSign,
                            b'a' ..= b'f' | b'A' ..= b'F' => state = State::HexExpDig,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    state = State::HexExpDig;
                                } else {
                                    return LiteralType::Real;
                                }
                            }
                        }
                    },

                    State::HexExpSign => {
                        match *self.peek(){
                            b'a' ..= b'f' | b'A' ..= b'F' => state = State::HexExpDig,
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    state = State::HexExpDig;
                                } else {
                                    return LiteralType::Real;
                                }
                            }
                        }
                    },

                    State::HexExpDig => {
                        match *self.peek(){
                            b'a' ..= b'f' | b'A' ..= b'F' => {},
                            _ => {
                                if CHAR_CLASS[*self.peek() as usize] == 2 {
                                    {};
                                } else {
                                    return LiteralType::Real;
                                }
                            }
                        }
                    }
                }
            }
            self.advance();
        }
    }

    fn detect_punc(&mut self) -> Punc {
        unsafe{
            let c = *self.peek();
            self.advance();
            match c{
                b'=' => {
                    if *self.peek() == b'>' {
                        self.advance();
                        return Punc::FatArrow;
                    } else {
                        return Punc::Assignment;
                    }
                }
                b'+' => return Punc::Plus,
                b'-' => {
                    if *self.peek() == b'>' {
                        self.advance();
                        return Punc::Arrow;
                    } else {
                        return Punc::Minus;
                    }
                }
                b'*' => return Punc::Mul,
                b'/' => return Punc::Div,
                b'%' => return Punc::Mod,
                b'.' => {
                    if *self.peek() == b'.' {
                        self.advance();
                        if *self.peek() == b'=' {
                            self.advance();
                            return Punc::RangeIncl;
                        } else {
                            return Punc::Range;
                        }
                    } else {
                        return Punc::Dot;
                    }
                }
                b'_' => return Punc::Wildcard,
                b':' => return Punc::Quantifier,
                b'^' => return Punc::Pointer,
                b'@' => return Punc::Address,
                b'[' => return Punc::BrackL,
                b']' => return Punc::BrackR,
                b'(' => return Punc::ParenL,
                b')' => return Punc::ParenR,
                b'{' => return Punc::BraceL,
                b'}' => return Punc::BraceR,
                b',' => return Punc::Comma,
                b';' => return Punc::Semicolon,
                b'<' => return Punc::GenL,
                b'>' => return Punc::GenR,
                b'"' => return Punc::Doublequote,   // Becomes string detector
                b'$' => return Punc::Dollar,        // Becomes string directive
                b'#' => {
                    if *self.peek() == b'#' {
                        self.advance();
                        while *self.peek() != b'\n' && *self.peek() != 0 {
                            self.advance();
                        }
                        return Punc::Docs;
                    } else {
                        while *self.peek() != b'\n' && *self.peek() != 0 {
                            self.advance();
                        }
                        return Punc::Comment;
                    }
                },
                _ => return Punc::Unknown,
            }
        }
    }
}