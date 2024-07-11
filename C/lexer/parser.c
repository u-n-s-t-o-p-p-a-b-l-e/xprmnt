#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

typedef enum {
	TOKEN_IF,
	TOKEN_ELSE,
	TOKEN_IDENTIFIER,
	TOKEN_LBRACE,
	TOKEN_RBRACE,
	TOKEN_ASSIGN,
	TOKEN_SEMICOLON,
	TOKEN_EOF,
	TOKEN_INVALID
} TokenType;

typedef struct {
	TokenType type;
	char *value;
} Token;

typedef struct {
	const char *input;
	size_t pos;
	size_t length;
	Token current_token;
} Lexer;
