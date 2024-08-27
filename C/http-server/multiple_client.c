#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <pthread.h>
#include <fcntl.h>
#include <sys/stat.h>

#define PORT 8080
#define BUFFER_SIZE 1024
#define MAX_CLIENTS 10

void *handle_client(void *socket_desc);
void send_file(int client_socket, const char *filename);

int main() {
    int server_fd, client_socket, *new_sock;
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    pthread_t thread_id;

    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    int opt = 1;
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
        perror("Setsockopt failed");
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, MAX_CLIENTS) < 0) {
        perror("Listen failed");
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d\n", PORT);

    while (1) {
        if ((client_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t*)&addrlen)) < 0) {
            perror("Accept failed");
            continue;
        }

        printf("Neww connection accepted\n");

        new_sock = malloc(sizeof(int));
        *new_sock = client_socket;

        if (pthread_create(&thread_id, NULL, handle_client, (void*)new_sock) < 0) {
            perror("Could not create thread");
            free(new_sock);
            continue;
        }

        pthread_detach(thread_id);
    }

    return 0;
}

void *handle_client(void *socket_desc) {
    int client_socket = *(int*)socket_desc;
    char buffer[BUFFER_SIZE] = {0};
    ssize_t bytes_read;

    bytes_read = read(client_socket, buffer, BUFFER_SIZE);
    if (bytes_read < 0) {
        perror("Read failed");
        goto cleanup;
    }

    char *token = strtok(buffer, " ");
    if (token == NULL || strcmp(token, "GET") != 0) {
        goto cleanup;
    }

    token = strtok(NULL, " ");
    if (token == NULL) {
        goto cleanup;
    }

    char *path = token + 1;
    if (strlen(path) == 0) {
        path =  "index.html";
    }

    send_file(client_socket, path);

cleanup:
    close(client_socket);
    free(socket_desc);
    return NULL;
}

void send_file(int client_socket, const char *filename) {
    char buffer[BUFFER_SIZE];
    char response_header[BUFFER_SIZE];
    int file_fd = open(filename, O_RDONLY);
    if (file_fd < 0) {
        snprintf(response_header, sizeof(response_header),
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n404 Not Found");
        send(client_socket, response_header, strlen(response_header), 0);
        return;
    }

    struct stat file_stat;
    fstat(file_fd, &file_stat);

    snprintf(response_header, sizeof(response_header),
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: %ld\r\n\r\n",
        file_stat.st_size);
    send(client_socket, response_header, strlen(response_header), 0);

    ssize_t bytes_read;
    while ((bytes_read = read(file_fd, buffer, sizeof(buffer))) > 0) {
        send(client_socket, buffer, bytes_read, 0);
    }

    close(file_fd);
}
