#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080
#define BUFFER_SIZE 1024

int main() {
    int sockfd;
    struct sockaddr_in server_address;
    char buffer[BUFFER_SIZE];
    socklen_t addrlen = sizeof(server_address);

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    server_address.sin_family = AF_INET;
    server_address.sin_port = htons(PORT);
    server_address.sin_addr.s_addr = INADDR_ANY;

    if (bind(sockfd, (struct sockaddr *)&server_address, sizeof(server_address)) < 0) {
        perror("Bind failed");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    while (1) {
    memset(buffer, 0, BUFFER_SIZE);
    recvfrom(sockfd, buffer, BUFFER_SIZE, 0, (struct sockaddr *)&server_address, &addrlen);
    printf("Received broadcast message: %s\n", buffer);
    }

    close(sockfd);
    return 0;
}
