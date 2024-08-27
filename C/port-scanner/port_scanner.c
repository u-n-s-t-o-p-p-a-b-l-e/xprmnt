#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

void scan_port(char *ip, int port) {
    int sock;
    struct sockaddr_in target;
    sock = socket(AF_INET, SOCK_STREAM, 0);

    if (sock < 0) {
        perror("Socket creation failed");
        return;
    }

    target.sin_family = AF_INET;
    target.sin_port = htons(port);
    target.sin_addr.s_addr = inet_addr(ip);

    if (connect(sock, (struct sockaddr *)&target, sizeof(target)) == 0) {
        printf("Port %d is open\n", port);
    }

    close(sock);
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        printf("Usage: %s <IP> <port-range>\n", argv[0]);
        return 1;
    }

    char *ip = argv[1];
    int start_port, end_port;
    sscanf(argv[2], "%d-%d", &start_port, &end_port);

    for (int port = start_port; port <= end_port; port++) {
        scan_port(ip, port);
    }

    return 0;
}
