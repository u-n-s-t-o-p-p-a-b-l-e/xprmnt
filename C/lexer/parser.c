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

Lexer create_lexer(const char *input) {
    Lexer lexer;
    lexer.input = input;
    lexer.pos = 0;
    lexer.length = strlen(input);
    lexer.current_token.type = TOKEN_INVALID;
    lexer.current_token.value = NULL;
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

typedef struct {
    Lexer lexer;
} Parser;

Parser create_parser(const char *input) {
    Parser parser;
    parser.lexer = create_lexer(input);
    parser.lexer.current_token = get_next_token(&parser.lexer);
    return parser;
}

void error(const char *message) {
    fprintf(stderr, "%s\n", message);
    exit(EXIT_FAILURE);
}

void match(Parser *parser, TokenType type) {
    if (parser->lexer.current_token.type == type) {
        free_token(parser->lexer.current_token);
        parser->lexer.current_token = get_next_token(&parser->lexer);
    } else {
        char error_msg[100];
        snprintf(error_msg, sizeof(error_msg), "Syntax error: unexpected token '%s', expected token type %d", parser->lexer.current_token.value, type);
        error(error_msg);
    }
}

void parse_expression(Parser *parser);
void parse_statement(Parser *parser);

void parse_expression(Parser *parser) {
    if (parser->lexer.current_token.type == TOKEN_IDENTIFIER) {
        match(parser, TOKEN_IDENTIFIER);
        if (parser->lexer.current_token.type == TOKEN_ASSIGN) {
            match(parser, TOKEN_ASSIGN);
            if (parser->lexer.current_token.type == TOKEN_IDENTIFIER) {
                match(parser, TOKEN_IDENTIFIER);
            } else {
                error("Syntax error: expected identifier after '='");
            }
        } else {
            error("Syntax error: expected '=' after identifier");
        }
    } else {
        error("Syntax error: expected identifier in expression");
    }
}

void parse_block(Parser *parser) {
    match(parser, TOKEN_LBRACE);
    while (parser->lexer.current_token.type != TOKEN_RBRACE && parser->lexer.current_token.type != TOKEN_EOF) {
        parse_statement(parser);
    }
    match(parser, TOKEN_RBRACE);
}

void parse_if_statement(Parser *parser) {
    match(parser, TOKEN_IF);
    parse_expression(parser);
    parse_block(parser);

    if (parser->lexer.current_token.type == TOKEN_ELSE) {
        match(parser, TOKEN_ELSE);
        parse_block(parser);
    }
}

void parse_statement(Parser *parser) {
    if (parser->lexer.current_token.type == TOKEN_IF) {
        parse_if_statement(parser);
    } else {
        parse_expression(parser);
        match(parser, TOKEN_SEMICOLON);
    }
}

void parse_program(Parser *parser) {
    while (parser->lexer.current_token.type != TOKEN_EOF) {
        parse_statement(parser);
    }
}

int main() {
    const char *input = "if x = y { doSomething = z; } else { doSomethingElse = z; }";
    Parser parser = create_parser(input);
    parse_program(&parser);
    printf("Parsing completed successfully.\n");
    return 0;
}

