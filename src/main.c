#include <stdio.h>
#include <stdlib.h>
#include "lex.h"

int main (){
    printf("Kern Compiler v0.0.1\n");

    FILE* file = fopen("examples/hello.asm", "rb");
    if(!file){
        fprintf(stderr, "Failed to open file.\n");
        return 1;
    }
    fseek(file, 0, SEEK_END);
    size_t file_size = ftell(file);
    rewind(file);

    char* buffer = malloc(file_size + 1);
    if (!buffer){
        fprintf(stderr, "Out of memory.\n");
        fclose(file);
        return 1;
    }

    if (fread(buffer, 1, file_size, file) != file_size){
        fprintf(stderr, "File read error.\n");
        free(buffer);
        fclose(file);
        return 1;
    }

    buffer[file_size] = '\0';
    fclose(file);
    printf("File loaded successfully. Filesize: %d bytes.\n", file_size);
    
    Lexer lexer = init_lex(buffer);
    TokenStream ts;
    init_tokstream(&ts);

    while (1) {
        Token token = next_token(&lexer);
        push_tokstream(&ts, token);
        if (token.type == TOK_EOF) break;
    }

    printf("%d tokens detected.\n", ts.len);

    free_tokstream(&ts);
    free(buffer);
    return 0;

}