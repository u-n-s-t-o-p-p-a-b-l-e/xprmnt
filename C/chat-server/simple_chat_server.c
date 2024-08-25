#include <arpa/inet.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define PORT 8080
#define MAX_CLIENTS 10
#define BUFFER_SIZE 1024

int clients[MAX_CLIENTS] = {0};

void *handle_client(void *client_socket) {
  int sock = *(int *)client_socket;
  char buffer[BUFFER_SIZE];

  while (1) {
    memset(buffer, 0, sizeof(buffer));
    int bytes_received = recv(sock, buffer, sizeof(buffer), 0);
    if (bytes_received <= 0) {
      printf("Client disconnected\n");
      close(sock);
      for (int i = 0; i < MAX_CLIENTS; i++) {
        if (clients[i] == sock) {
          clients[i] = 0;
          break;
        }
      }
      break;
    }

    printf("Received: %s", buffer);

    for (int i = 0; i < MAX_CLIENTS; i++) {
      if (clients[i] != 0 && clients[i] != sock) {
        send(clients[i], buffer, strlen(buffer), 0);
      }
    }
  }

  return NULL;
}

int main() {
  int server_socket = socket(AF_INET, SOCK_STREAM, 0);
  struct sockaddr_in server_address = {.sin_family = AF_INET,
                                       .sin_port = htons(PORT),
                                       .sin_addr.s_addr = INADDR_ANY};

  bind(server_socket, (struct sockaddr *)&server_address,
       sizeof(server_address));
  listen(server_socket, MAX_CLIENTS);

  printf("Chat server running on port %d\n", PORT);

  while (1) {
    int client_socket = accept(server_socket, NULL, NULL);
    printf("client connected\n");

    for (int i = 0; i < MAX_CLIENTS; i++) {
      if (clients[i] == 0) {
        clients[i] = client_socket;
        pthread_t thread;
        pthread_create(&thread, NULL, handle_client, &client_socket);
        break; 
      }
    }
  }

  close(server_socket);
  return 0;
}


