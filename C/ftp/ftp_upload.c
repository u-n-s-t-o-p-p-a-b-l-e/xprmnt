#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define BUFFER_SIZE 1024

void upload_file(int sock, const char *filename) {
    FILE *file = fopen(filename, "rb");
    if (!file) {
        perror("Failed to open file");
        return;
    }

    char buffer[BUFFER_SIZE];
    while (1) {
        size_t bytes_read = fread(buffer, 1, BUFFER_SIZE, file);
        if (bytes_read > 0) {
            send(sock, buffer, bytes_read, 0);
        }
        if (bytes_read < BUFFER_SIZE) {
        if (feof(file)) {
            printf("File upload complete\n");
        }
        if (ferror(file)) {
            perror("Error reading file");
        }
        break;
        }
    }

    fclose(file);
}

int main() {
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in server_address = {
        .sin_family = AF_INET,
        .sin_port = htons(21),
        .sin_addr.s_addr = inet_addr("127.0.0.1")
    };

    connect(sock, (struct sockaddr *)&server_address, sizeof(server_address));

    char response[BUFFER_SIZE];
    recv(sock, response, sizeof(response), 0);
    printf("Server Response: %s", response);

    char *user = "USER anonymous\r\n";
    send(sock, user, strlen(user), 0);
    recv(sock, response, sizeof(response), 0);
    printf("Server Response: %s", response);

    char *pass = "PASS anonymous\r\n";
    send(sock, pass, strlen(pass), 0);
    recv(sock, response, sizeof(response), 0);
    printf("Server Response: %s", response);

    char *stor = "STOR upload.txt\r\n";
    send(sock, stor, strlen(stor), 0);
    recv(sock, response, sizeof(response), 0);
    printf("Server Response: %s", response);

    upload_file(sock, "upload.txt");

    close(sock);
    return 0;
}
