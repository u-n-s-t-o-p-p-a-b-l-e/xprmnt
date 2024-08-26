#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <pthread.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void *handle_client(void *client_socket) {
    int sock = *(int *)client_socket;
    char buffer[BUFFER_SIZE];
    char response[BUFFER_SIZE];

    read(sock, buffer, BUFFER_SIZE);
    printf("Request: %s\n", buffer);

    snprintf(response, sizeof(response), "HTTP/1.1 200 OK\r\n"
                                         "Content-Length: %ld\r\n"
                                         "Content-Type: text/html\r\n\r\n"
                                         "<html><body><h1>Hey, There!</h1></body></html>",
                                         strlen("<html><body><h1>Hey, there!</h1></body></html>"));

    write(sock, response, strlen(response));
    close(sock);
    return NULL;
}

int main() {
    int server_socket, client_socket;
    struct sockaddr_in server_address, client_address;
    socklen_t client_length = sizeof(client_address);

    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    server_address.sin_family = AF_INET;
    server_address.sin_port = htons(PORT);
    server_address.sin_addr.s_addr = INADDR_ANY;

    if (bind(server_socket, (struct sockaddr *)&server_address, sizeof(server_address)) < 0) {
        perror("Bind failed");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    if (listen(server_socket, 5) < 0) {
        perror("Listen failed");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d\n", PORT);

    while (1) {
    client_socket = accept(server_socket, (struct sockaddr *)&client_address, &client_length);
        if (client_socket < 0) {
            perror("Client accept failed");
            continue;
        }

        pthread_t thread;
        pthread_create(&thread, NULL, handle_client, &client_socket);
    }

    close(server_socket);
    return 0;
}
