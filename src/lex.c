#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <ctype.h>
#include "lex.h"

Lexer init_lex(const char* source) {
    return (Lexer){
        .source = source,
        .src_len = strlen(source),
        .src_pos = 0,
        .src_line = 1,
        .src_col = 1
    };
}

char peek_char(Lexer* lexer){
    if (lexer->src_pos >= lexer->src_len) return '\0';
    return lexer->source[lexer->src_pos];
}

void advance(Lexer* lexer){
    if (lexer->src_pos >= lexer->src_len)
        return;
    char c = lexer->source[lexer->src_pos++];
    if (c == '\n') {
        lexer->src_line++;
        lexer->src_col = 1;
    } else {
        lexer->src_col++;
    }
}

void skip_nontokens(Lexer* lexer){
    while (1){
        char c = peek_char(lexer);

        if (c == ' ' || c == '\t' || c == '\r') {
            advance(lexer);
        } else {
            break;
        } 
    }
}

Token next_token(Lexer* lexer) {
    skip_nontokens(lexer);

    size_t start = lexer->src_pos;
    int line = lexer->src_line;
    int col = lexer->src_col;

    if (lexer->src_pos >= lexer->src_len)
        return (Token){TOK_EOF, start, 0, line, col};

    char c = peek_char(lexer);

    if (isalpha(c) || c == '_'){
        detect_ident(lexer);
        return (Token){TOK_IDENT, start, lexer->src_pos - start, line, col};
    }

    if ((isdigit(c))){
        detect_number(lexer);
        return (Token){TOK_NUMBER, start,lexer->src_pos - start, lexer->src_line, lexer->src_col};
    }

    switch (c) {
        case '(':
            advance(lexer);
            return (Token){TOK_PARENL, start, 1, line, col};
        case ')':
            advance(lexer);
            return (Token){TOK_PARENR, start, 1, line, col};
        case '+':
            advance(lexer);
            return (Token){TOK_PLUS, start, 1, line, col};
        case '-':
            advance(lexer);
            return (Token){TOK_MINUS, start, 1, line, col};
        case ';':
            while (peek_char(lexer) != '\n' && peek_char(lexer) != '\0')
                advance(lexer);
            return (Token){TOK_COMMENT, start, lexer->src_pos - start, line, col};
        case '"':
            do{
                advance(lexer);
            } while (peek_char(lexer) != '"' && peek_char(lexer) != '\n' && peek_char(lexer) != '\0');
            return (Token){TOK_STRING, start, lexer->src_pos - start, line, col};
        default:
            advance(lexer);
            return (Token){TOK_UNKNOWN, start, 1, line, col};
        }
}

void detect_number(Lexer *lexer) {
    typedef enum {
        START, STARTZERO,
        DEC, DECDOT, DECFRAC, DECEXP, DECEXPSIGN, DECEXPDIG,
        BIN, OCT,
        HEX, HEXDOT, HEXFRAC, HEXEXP, HEXEXPSIGN, HEXEXPDIG,
        END
    } State;

    State state = START;

    while (state != END) {
        char c = peek_char(lexer);
        if (c == '\0') break;

        switch (state) {

        case START:
            if (c == '0') { state = STARTZERO; advance(lexer); }
            else if (isdigit(c)) { state = DEC; advance(lexer); }
            else state = END;
            break;

        case STARTZERO:
            if (c == 'b' || c == 'B') { state = BIN; advance(lexer); }
            else if (c == 'o' || c == 'O') { state = OCT; advance(lexer); }
            else if (c == 'x' || c == 'X') { state = HEX; advance(lexer); }
            else if (isdigit(c)) { state = DEC; advance(lexer); }
            else if (c == '.') { state = DECDOT; advance(lexer); }
            else if (c == 'e' || c == 'E') { state = DECEXP; advance(lexer); }
            else state = END;
            break;

        case DEC:
            if (isdigit(c)) advance(lexer);
            else if (c == '.') { state = DECDOT; advance(lexer); }
            else if (c == 'e' || c == 'E') { state = DECEXP; advance(lexer); }
            else state = END;
            break;

        case DECDOT:
            if (isdigit(c)) { state = DECFRAC; advance(lexer); }
            else state = END;
            break;

        case DECFRAC:
            if (isdigit(c)) advance(lexer);
            else if (c == 'e' || c == 'E') { state = DECEXP; advance(lexer); }
            else state = END;
            break;

        case DECEXP:
            if (c == '+' || c == '-') { state = DECEXPSIGN; advance(lexer); }
            else if (isdigit(c)) { state = DECEXPDIG; advance(lexer); }
            else state = END;
            break;

        case DECEXPSIGN:
            if (isdigit(c)) { state = DECEXPDIG; advance(lexer); }
            else state = END;
            break;

        case DECEXPDIG:
            if (isdigit(c)) advance(lexer);
            else state = END;
            break;

        case BIN:
            if (c == '0' || c == '1') advance(lexer);
            else state = END;
            break;

        case OCT:
            if (c >= '0' && c <= '7') advance(lexer);
            else state = END;
            break;

        case HEX:
            if (isxdigit(c)) advance(lexer);
            else if (c == '.') { state = HEXDOT; advance(lexer); }
            else if (c == 'p' || c == 'P') { state = HEXEXP; advance(lexer); }
            else state = END;
            break;

        case HEXDOT:
            if (isxdigit(c)) { state = HEXFRAC; advance(lexer); }
            else state = END;
            break;

        case HEXFRAC:
            if (isxdigit(c)) advance(lexer);
            else if (c == 'p' || c == 'P') { state = HEXEXP; advance(lexer); }
            else state = END;
            break;

        case HEXEXP:
            if (c == '+' || c == '-') { state = HEXEXPSIGN; advance(lexer); }
            else if (isdigit(c)) { state = HEXEXPDIG; advance(lexer); }
            else state = END;
            break;

        case HEXEXPSIGN:
            if (isdigit(c)) { state = HEXEXPDIG; advance(lexer); }
            else state = END;
            break;

        case HEXEXPDIG:
            if (isdigit(c)) advance(lexer);
            else state = END;
            break;

        case END:
            advance(lexer); return;
        }
    }
}

void detect_ident(Lexer *lexer){
    while (isalnum(peek_char(lexer)) || peek_char(lexer) == '_'){
        advance(lexer);
    }
}

void init_tokstream(TokenStream* ts){
    ts->len = 0;
    ts->cap = 16;
    ts->data = malloc(ts->cap * sizeof(Token));
}

void push_tokstream(TokenStream* ts, Token tok){
    if (ts->len >= ts->cap){
        ts->cap *= 2;
        ts->data = realloc(ts->data, ts->cap * sizeof(Token));
        if (!ts->data)
            exit(1);
    }
    ts->data[ts->len++] = tok;
}

void free_tokstream(TokenStream* ts){
    free(ts->data);
    ts->data = NULL;
    ts->len = 0;
    ts->cap = 0;
}