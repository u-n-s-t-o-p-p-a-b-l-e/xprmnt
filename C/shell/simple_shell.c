#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

#define MAX_INPUT 1024
#define MAX_ARGS 64

void execute_command(char *args[]) {
    pid_t pid = fork();

    if (pid == 0) {
        if (execvp(args[0], args) == -1) {
            perror("Error executing command");
            exit(EXIT_FAILURE);
        }
    } else if (pid < 0) {
        perror("Error forking");
    } else {
        int status;
        waitpid(pid, &status, 0);
    }
}

void parse_input(char *input, char *args[]) {
    char *token = strtok(input, " \t\n");
    int i = 0;
    while (token != NULL && i < MAX_ARGS - 1) {
        args[i++] = token;
        token = strtok(NULL, " \t\n");
    }
    args[i] = NULL;
}

int main() {
    char input[MAX_INPUT];
    char *args[MAX_ARGS];

    while (1) {
        printf("simple_shell> ");
        if (fgets(input, sizeof(input), stdin) == NULL) {
            break;
        }

        input[strcspn(input, "\n")] = 0;

        if (strcmp(input, "exit") == 0) {
            break;
        }

        parse_input(input, args);
        if (args[0] != NULL) {
            execute_command(args);
        }
    }

    return 0;
}
