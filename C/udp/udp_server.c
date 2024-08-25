#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define PORT 8080
#define BUFFER_SIZE 1024

int main() {
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  struct sockaddr_in server_address = {.sin_family = AF_INET,
                                       .sin_port = htons(PORT),
                                       .sin_addr.s_addr = INADDR_ANY};

  bind(sockfd, (struct sockaddr *)&server_address, sizeof(server_address));

  char buffer[BUFFER_SIZE];
  struct sockaddr_in client_address;
  socklen_t client_length = sizeof(client_address);

  printf("UDP server listening on port %d\n", PORT);

  while (1) {
    memset(buffer, 0, sizeof(buffer));
    recvfrom(sockfd, buffer, sizeof(buffer), 0,
             (struct sockaddr *)&client_address, &client_length);
    printf("Received: %s\n", buffer);

    sendto(sockfd, buffer, strlen(buffer), 0,
           (struct sockaddr *)&client_address, client_length);
  }

  close(sockfd);
  return 0;
}
