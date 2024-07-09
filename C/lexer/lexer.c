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
} lexer;

Lexer create_lexer(const char *input) {
	Lexer lexer;
	lexer.input = input;
	lexer.pos = 0;
	lexer.length = strlen(input);
	return lexer;
}

char peek(Lexer *lexer) {
	if (lexer->pos < lexer->length) {
		return lexer->input[lexer->pos];
	}
	return '\0';
}

char advance(Lexer *lexer) {
	if (lexer->pos < lexer->length) {
		return lexer->input[lexer->pos++];
	}
	
	return '\0';
}

Token create_token(TokenType type, const char *value) {
	Token token;
	token.type = type;
	token.value = strdup(value);
	return token;
}

void get_next_token(Lexer *lexer) {
	while (lexer->pos < lexer->length) {
		char current = peek(lexer);
	}
}
