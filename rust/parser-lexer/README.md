Simple lexer and parser for parsing a basic subset of a non-numerical expression involving identifiers and basic keywords. This example will use a lexer to tokenize an input string and a parser to build a basic abstract syntax tree (AST).

For lexer.rs, run it with:

```
 rustc lexer.rs -o lexer && ./lexer
```

For parser.rs:

```
rustc parser.rs -o parser && ./parser
```
