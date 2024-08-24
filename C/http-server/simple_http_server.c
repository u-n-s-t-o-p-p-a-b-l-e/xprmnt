#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void serve_file(int client_socket, const char *filename) {
  FILE *file = fopen(filename, "r");
  if (!file) {
    const char *not_found_response = "HTTP/1.1 404 Not Found\r\n"
                                     "Content-Length: 13\r\n"
                                     "Content-Type: text/plain\r\n\r\n"
                                     "404 Not Found";
    send(client_socket, not_found_response, strlen(not_found_response), 0);
    return;
  }

  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *file_content = malloc(file_size);
  fread(file_content, 1, file_size, file);
  fclose(file);

  char header[BUFFER_SIZE];
  snprintf(header, sizeof(header),
           "HTTP/1.1 200 OK\r\n"
           "Content-Length: %ld\r\n"
           "Content-Type: text/plain\r\n\r\n",
           file_size);

  send(client_socket, header, strlen(header), 0);
  send(client_socket, file_content, file_size, 0);

  free(file_content);
}

int main() {
  int server_socket = socket(AF_INET, SOCK_STREAM, 0);
  struct sockaddr_in server_address = {.sin_family = AF_INET,
                                       .sin_port = htons(PORT),
                                       .sin_addr.s_addr = INADDR_ANY
                                       };
    bind(server_socket, (struct sockaddr *)&server_address, sizeof(server_address));
    listen(server_socket, 10);

    printf("HTTP server running on port %d\n", PORT);

    while (1) {
    int client_socket = accept(server_socket, NULL, NULL);

    char request[BUFFER_SIZE];
    recv(client_socket, request, sizeof(request), 0);
    printf("Received request:\n%s", request);

    char *method = strtok(request, " ");
    char *path = strtok(NULL, " ");

    if (strcmp(method, "GET") == 0 && path != NULL) {
    if (strcmp(path, "/") == 0) {
    path = "/index.html";
    }
    serve_file(client_socket, path + 1);
    } else {
    const char *bad_request_response = "HTTP/1.1 400 Bad Request\r\n"
                                       "Content-Length: 15\r\n"
                                       "Content-Type: text/plain\r\n\r\n"
                                       "400 Bad Request";
    send(client_socket, bad_request_response, strlen(bad_request_response), 0);
    }

    close(client_socket);
    }

    close(server_socket);
    return 0;
}
