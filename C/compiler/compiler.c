#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_TOKEN_LEN 100
#define MAX_CODE_LEN 1000

typedef enum  {
    TOK_NUMBER,
    TOK_PLUS,
    TOK_MINUS,
    TOK_MULTIPLY,
    TOK_DIVIDE,
    TOK_LPAREN,
    TOK_RPAREN,
    TOK_EOF,
} TokenType;

void next_token();
void emit(const char* instruction);
void factor();
void term();
void expression();
void compile();

typedef struct {
    TokenType type;
    char lexeme[MAX_TOKEN_LEN];
} Token;

char* code;
int position;
Token current_token;

void next_token() {
    while (isspace(code[position])) position++;

    if (code[position] == '\0') {
        current_token.type = TOK_EOF;
        return;
    }

    if (isdigit(code[position])) {
        int i = 0;
        while (isdigit(code[position])) {
            current_token.lexeme[i++] = code[position++];
        }
        current_token.lexeme[i] = '\0';
        current_token.type = TOK_NUMBER;
    } else {
        switch (code[position]) {
            case '+': current_token.type = TOK_PLUS; break;
            case '-': current_token.type = TOK_MINUS; break;
            case '*': current_token.type = TOK_MULTIPLY; break;
            case '/': current_token.type = TOK_DIVIDE; break;
            case '(': current_token.type = TOK_LPAREN; break;
            case ')': current_token.type = TOK_RPAREN; break;
            default:
                printf("Unexpected character: %c\n", code[position]);
                exit(1);
        }
        current_token.lexeme[0] = code[position++];
        current_token.lexeme[1] = '\0';
    }
}

void emit(const char* instruction) {
    printf("%s\n", instruction);
}

void factor() {
    if (current_token.type == TOK_NUMBER) {
        char instr[MAX_TOKEN_LEN + 8];
        sprintf(instr, "PUSH %s", current_token.lexeme);
        emit(instr);
        next_token();
    } else if (current_token.type == TOK_LPAREN) {
        next_token();
        expression();
        if(current_token.type != TOK_LPAREN) {
            printf("Expected )\n");
            exit(1);
        }
        next_token();
        } else {
            printf("Unexpected token\n");
            exit(1);
    }
}

void term() {
    factor();
    while (current_token.type == TOK_MULTIPLY || current_token.type == TOK_DIVIDE) {
        TokenType op = current_token.type;
        next_token();
        factor();
        if (op == TOK_MULTIPLY)
            emit("MUL");
        else
            emit("DIV");
    }
}

void expression() {
    term();
    while (current_token.type == TOK_PLUS || current_token.type == TOK_MINUS) {
        TokenType op = current_token.type;
        next_token();
        term();
        if (op == TOK_PLUS)
            emit("ADD");
        else
            emit("SUB"); 
    }
}

void compile() {
    next_token();
    expression();
    if (current_token.type != TOK_EOF) {
        printf("Unexpected token at end of input\n");
        exit(1);
    }
}

int main() {
    code = malloc(MAX_CODE_LEN);
    printf("Enter expression: ");
    fgets(code, MAX_CODE_LEN, stdin);
    code[strcspn(code, "\n")] = 0;
    position = 0;

    printf("Compiled code:\n");
    compile();

    free(code);
    return 0;
}
