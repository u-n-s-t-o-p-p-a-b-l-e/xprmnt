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
} Lexer;

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

void free_token(Token token) {
	free(token.value);
}

Token get_next_token(Lexer *lexer) {
	while (lexer->pos < lexer->length) {
		char current = peek(lexer);

		if (isspace(current)) {
			advance(lexer);
			continue;
		}

		if (isalpha(current)) {
			size_t start = lexer->pos;
			while (isalpha(peek(lexer))) {
				advance(lexer);
			}
			size_t length = lexer->pos - start;
			char *value = strndup(lexer->input + start, length);

			if (strcmp(value, "if") == 0) {
				return create_token(TOKEN_IF, value);
			} else if (strcmp(value, "else") == 0) {
				return create_token(TOKEN_ELSE, value);
			} else {
				return create_token(TOKEN_IDENTIFIER, value);
			}
		}

		if (current == '{') {
			advance(lexer);
			return create_token(TOKEN_LBRACE, "{");
		}

		if (current == '}') {
			advance(lexer);
			return create_token(TOKEN_RBRACE, "}");
		}

		if (current == '=') {
			advance(lexer);
			return create_token(TOKEN_ASSIGN, "=");
		}

		if (current == ';') {
			advance(lexer);
			return create_token(TOKEN_SEMICOLON, ";");
		}

		advance(lexer);
		return create_token(TOKEN_INVALID, "INVALID");
	}

	return create_token(TOKEN_EOF, "EOF");
}

void print_token(Token token) {
	switch (token.type) {
		case TOKEN_IF:
			printf("TOKEN_IF: %s\n", token.value);
			break;
		case TOKEN_ELSE:
			printf("TOKEN_ELSE: %s\n", token.value);
			break;
		case TOKEN_IDENTIFIER:
			printf("TOKEN_IDENTIFIER: %s\n", token.value);
			break;
		case TOKEN_LBRACE:
			printf("TOKEN_LBRACE: %s\n", token.value);
			break;
		case TOKEN_RBRACE:
			printf("TOKEN_RBRACE: %s\n", token.value);
			break;
		case TOKEN_ASSIGN:
			printf("TOKEN_ASSIGN: %s\n", token.value);
			break;
		case TOKEN_SEMICOLON:
			printf("TOKEN_SEMICOLON: %s\n", token.value);
			break;
		case TOKEN_EOF:
			printf("TOKEN_EOF: %s\n", token.value);
			break;
		case TOKEN_INVALID:
			printf("TOKEN_INVALID: %s\n", token.value);
			break;
	}
}

int main() {
	const char *input = "if x = y { doSomething(); } else { doSomethingElse(); }";
	Lexer lexer = create_lexer(input);

	Token token;
	do {
		token = get_next_token(&lexer);
		print_token(token);
		free_token(token);;;
	} while (token.type != TOKEN_EOF);

	return 0;
}
