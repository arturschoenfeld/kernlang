# Compiler Design

Kern aims to utilize demand-driven compilation in which the source code gets broken down into semantic units that can query for other semantic units. Those will be called fragments.

Even in today's incremental compilers, the translation of individual source files gets executed through a single-pass pipeline of lexing, parsing, AST and IR generation and codegen. Retranslations can be triggered when changes have been made to a piece of code that neither change the semantic nor the behavior, such as a typo fix in the comments. Admittedly, some compilers do inbetween checks to reduce operations, but Kern performs those checks at every level to avoid redundant operations that lead to no changes.

## Lexing

The first stage of the translation is the lexical analysis, also known as tokenization. The source code gets broken down into atomic code units, so-called tokens. Tokens can be keywords, operators, variables and identifiers for instance.

Before actually translating something, the whole content of a source code file gets hashed and compared with the stored value of the source hash, if existent. When there is a different hash or none at all, tokenization begins. Then, the contents of the token stream gets hashed and compared with its corresponding hash value. If the stream hash is identical, it means that the semantics did not change. Otherwise, the stream gets passed to the parser.

## Parsing

The parser analyzes the token stream and performs grammatical and type checks. While most compilers would directly generate an Abstract Syntax Tree of a whole file, Kern's parser bundles parts of the token stream into fragments. Fragments are individual code segments that contain isolated semantics and sequences. So far, fragments can be created from top-level definitions (variables, functions, modules), bundled data (structs, enums, unions, packs) and control flow statements (if, match, all loops).