#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <fcntl.h>
#include <errno.h>

#define MAX_EVENTS 10
#define BUFFER_SIZE 1024
#define PORT 8080

void set_nonblocking(int sock) {
    int flags = fcntl(sock, F_GETFL, 0);
    fcntl(sock, F_SETFL, flags | O_NONBLOCK);
}

void handle_connection(int client_socket) {
    char buffer[BUFFER_SIZE];
    ssize_t bytes_read = read(client_socket, buffer, BUFFER_SIZE - 1);
    if (bytes_read > 0) {
        buffer[bytes_read] = '\0';
        printf("Received: %s\n", buffer);

        char *response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHi, There!";
        write(client_socket, response, strlen(response));
    }
    close(client_socket);
}

int main() {
    int server_socket, client_socket, epoll_fd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    struct epoll_event ev, events[MAX_EVENTS];

    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    bind(server_socket, (struct sockaddr*)&server_addr, sizeof(server_addr));
    listen(server_socket, SOMAXCONN);
    set_nonblocking(server_socket);

    epoll_fd = epoll_create1(0);
    ev.events = EPOLLIN;
    ev.data.fd = server_socket;
    epoll_ctl(epoll_fd, EPOLL_CTL_ADD, server_socket, &ev);

    printf("Server listening on port %d\n", PORT);

    while (1) {
    int nfds = epoll_wait(epoll_fd, events, MAX_EVENTS, -1);
    for (int n = 0; n < nfds; ++n) {
        if (events[n].data.fd == server_socket) {
            client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_len);
            set_nonblocking(client_socket);
            ev.events = EPOLLIN | EPOLLET;
            ev.data.fd = client_socket;
            epoll_ctl(epoll_fd, EPOLL_CTL_ADD, client_socket, &ev);
            } else {
                handle_connection(events[n].data.fd);
            }
        }
    }

    close(server_socket);
    close(epoll_fd);
    return 0;
}
