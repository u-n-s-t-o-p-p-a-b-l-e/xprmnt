The lexer will tokenize the input string into keywords (if, else), identifiers, and symbols ({, }, =, ;).


Compile & run it with:

```
gcc lexer.c -o lexer && ./lexer
```

The parser will build a structure based on the tokens produced by the lexer. This will handle simple if-else statements.
Compile and run it with:

```
gcc parser.c -o parser && ./parser
```
