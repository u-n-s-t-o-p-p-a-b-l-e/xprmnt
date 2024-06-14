#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <pthread.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void *handle_client(void *arg) {
	int new_socket = *((int *)arg);
	char buffer[BUFFER_SIZE] = {0};
	char *welcome_message = "Welcome to the chat server!\n";

	send(new_socket, welcome_message, strlen(welcome_message), 0);
	printf("Welcome message sent\n");

	while (1) {
		memset(buffer, 0, BUFFER_SIZE);
		int valread = read(new_socket, buffer, BUFFER_SIZE);
		if (valread > 0) {
			printf("Client: %s", buffer);
		}
	}

	close(new_socket);
	return NULL;
}

void *send_messages(void *arg) {
	int new_socket = *((int *)arg);
	char buffer[BUFFER_SIZE] = {0};

	while (1) {
		printf("Server: ");
		fgets(buffer, BUFFER_SIZE, stdin);
		send(new_socket, buffer, strlen(buffer), 0);
	}

	return NULL;
}

int main() {
	int server_fd, new_socket;
	struct sockaddr_in addrress;
	int addrlen = sizeof(address);
	pthread_t client_thread, send_thread;

	if ((server_fd == socket(AF_INET, SOCK_STREAM, 0)) == 0) {
		perror("socket failed");
		exit(EXIT_FAILURE);
	}

	address.sin_family = AF_INET;
	address.sin_addr.s_addr = INADDR_ANY;
	address.sin_port = htons(PORT);

	if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
		perror("bind failed");
		close(server_fd);
		exit(EXIT_FAILURE);
	}

	if (listen(server_fd, 3) < 0) {
		perror("listen failed");
		close(server_fd);
		exit(EXIT_FAILURE);
	}
}
