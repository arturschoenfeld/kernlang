#ifndef LEX_H
#define LEX_H

#include <stdint.h>

typedef enum {
    TOK_EOF,
    TOK_IDENT,
    TOK_KEYWORD,
	TOK_LITERAL,
	TOK_OP,
	TOK_PUNC,
	TOK_ATTRIBUTE,		// Future feature
    TOK_NEWLINE,
    TOK_COMMENT,
    TOK_UNKNOWN
} TokenType;

typedef enum{
	// Data Types
	KWRD_ISIZE,
	KWRD_I8,
	KWRD_I16,
	KWRD_I32,
	KWRD_I64,
	KWRD_I128,
	KWRD_USIZE,
	KWRD_U8,
	KWRD_U16,
	KWRD_U32,
	KWRD_U64,
	KWRD_U128,
	KWRD_FIXSIZE,
	KWRD_FIX8,
	KWRD_FIX16,
	KWRD_FIX32,
	KWRD_FIX64,
	KWRD_FIX128,
	KWRD_FLT32,
	KWRD_FLT64,
	KWRD_FLT128,
	KWRD_BFLT16,
	KWRD_BOOL,

	// Literals
	KWRD_NULL,
	KWRD_TRUE,
	KWRD_FALSE,

	// Control Flow
	KWRD_IF,
	KWRD_ELSE,
	KWRD_MATCH,
	KWRD_LOOP,
	KWRD_FOR,
	KWRD_WHILE,
	KWRD_BREAK,
	KWRD_SKIP,
	KWRD_RETURN,
	KWRD_DEFER,

	// Contracts
	KWRD_PRE,
	KWRD_INV,
	KWRD_POST,

	// Bundled Data
	KWRD_STRUCT,
	KWRD_UNION,
	KWRD_ENUM,
	KWRD_PACK,

	// Top-Level Definitions
	KWRD_LET,
	KWRD_FN,
	KWRD_USE,

	// Intermediate Clauses
	KWRD_AS,
	KWRD_BE,
	KWRD_TO,
	KWRD_IN,
	KWRD_OF,

	// Qualifiers
	KWRD_CONST,
	KWRD_PUB,
	KWRD_VOL,
	KWRD_COMPTIME,		// Needs review

	// Tests
	KWRD_TEST,
	KWRD_ASSERT,

	// Coroutines
	KWRD_COROUT,
	KWRD_RUN,
	KWRD_YIELD,
	KWRD_CANCEL,

	// Interop
	KWRD_ASM,
	KWRD_EXPORT,
	KWRD_EXTERN,

	// Casts
	KWRD_BITCAST,
	KWRD_VALCAST,

	// Type Introspection and Layout
	KWRD_ALIGN,
	KWRD_SIZE,
	KWRD_TYPE		// Needs review
} Keyword;

typedef enum{
	LIT_INT,
	LIT_REAL,
	LIT_CHAR,
	LIT_STRING
} LiteralType;

typedef enum{
	OP_ASG,
	OP_ASGPLUS,
	OP_ASGMINUS,
	OP_ASGMUL,
	OP_ASGDIV,
	OP_ASGMOD,
	OP_ASGBITAND,
	OP_ASGBITOR,
	OP_ASGBITXOR,
	OP_ASGBITSHL,
	OP_ASGBITSHR,
	OP_PLUS,			// add or unary positive
	OP_MINUS,			// sub or unary negative
	OP_STAR,			// mul or dereference
	OP_DIV,
	OP_MOD,
	OP_AMP,				// bitwise and or reference
	OP_BITNOT,
	OP_BITOR,
	OP_BITXOR,
	OP_BITSHL,
	OP_BITSHR,
	OP_LOGNOT,
	OP_LOGAND,
	OP_LOGOR,
	OP_EQ,
	OP_NEQ,
	OP_LT,
	OP_LTE,
	OP_GT,
	OP_GTE,
	OP_DOT,
	OP_ARROW,			// Function return type
	OP_FATARROW,		// Match arm
	OP_RANGE,			// ..
	OP_RANGEINCL,		// ..=
} OperatorType;

typedef enum{
	PUNC_SEMICOLON,		// Mandatory for statement separation in the same line
	PUNC_BRACEL,
	PUNC_BRACER,
	PUNC_BRACKL,
	PUNC_BRACKR,
	PUNC_PARENL,
	PUNC_PARENR,
	PUNC_HASH,			// Comments
	PUNC_DOCS			// ##
} PuncType;

typedef union{
	Keyword keyword;
	LiteralType literal;
	OperatorType operator;	
	PuncType punc;
} TokenLiteral;

typedef struct {
    TokenType type;
	TokenLiteral token_literal;
    size_t src_start;
    size_t src_len;
    int line;
    int col;
} Token;

typedef struct {
    const char *source;
    size_t src_pos;
    size_t src_len;
    int src_line;
    int src_col;
} Lexer;

typedef struct {
    Token* data;
    size_t len;
    size_t cap;
} TokenStream;

Lexer init_lex(const char *source);

char peek_char(Lexer* lexer);

void advance(Lexer* lexer);

void skip_nontokens(Lexer* lexer);

Token next_token(Lexer *lexer);

void detect_number(Lexer *lexer);

void detect_ident(Lexer *lexer);

void init_tokstream(TokenStream* ts);

void push_tokstream(TokenStream* ts, Token tok);

void free_tokstream(TokenStream* ts);

#endif