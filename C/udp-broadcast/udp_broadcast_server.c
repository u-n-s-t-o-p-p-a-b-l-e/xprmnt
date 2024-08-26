#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080
#define BROADCAST_IP "255.255.255.255"
#define BUFFER_SIZE 1024

int main() {
    int sockfd;
    struct sockaddr_in broadcast_address;
    char *message = "Hello, Broadcast!";

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    int broadcast_enable = 1;
    if (setsockopt(sockfd, SOL_SOCKET, SO_BROADCAST, &broadcast_enable, sizeof(broadcast_enable)) < 0) {
        perror("Error in setting broadcast option");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    broadcast_address.sin_family = AF_INET;
    broadcast_address.sin_port = htons(PORT);
    broadcast_address.sin_addr.s_addr = inet_addr(BROADCAST_IP);

    while (1) {
        sendto(sockfd, message, strlen(message), 0, (struct sockaddr *)&broadcast_address, sizeof(broadcast_address));
        printf("Broadcast message sent: %s]n", message);
        sleep(2);
    }

    close(sockfd);
    return 0;
}
