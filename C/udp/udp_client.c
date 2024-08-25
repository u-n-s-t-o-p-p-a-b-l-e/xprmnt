#include <arpa/inet.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define PORT 8080
#define BUFFER_SIZE 1024

int main() {
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  struct sockaddr_in server_address = {.sin_family = AF_INET,
                                       .sin_port = htons(PORT),
                                       .sin_addr.s_addr =
                                           inet_addr("127.0.0.1")};

  char buffer[BUFFER_SIZE];
  printf("Enter message: ");
  fgets(buffer, BUFFER_SIZE, stdin);

  sendto(sockfd, buffer, strlen(buffer), 0, (struct sockaddr *)&server_address,
         sizeof(server_address));

  memset(buffer, 0, sizeof(buffer));
  recvfrom(sockfd, buffer, sizeof(buffer), 0, NULL, NULL);
  printf("Server response: %s\n", buffer);

  close(sockfd);
  return 0;
}
