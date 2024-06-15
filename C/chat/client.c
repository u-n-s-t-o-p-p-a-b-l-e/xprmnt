#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <pthread.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void *receive_messages(void *arg) {
	int sock = *((int *)arg);
	char buffer[BUFFER_SIZE] = {0};

	while (1) {
		memset(buffer, 0, BUFFER_SIZE);
		int valread = read(sock, buffer, BUFFER_SIZE);
		if (valread > 0) {
			printf("Server: %s", buffer);
		}
	}

	return NULL;
}

int main() {
	int sock = 0;
	struct sockaddr_in serv_addr;
	char buffer[BUFFER_SIZE] = {0};
	pthread_t receive_thread;

	if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
		perror("Socket creation error");
		return -1;
	}

	serv_addr.sin_family = AF_INET;
	serv_addr.sin_port = htons(PORT);

	if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
		perror("Invalid address/ Address not supported");
		return -1;
	}

	if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
		perror("Connection failed");
		return -1;
	}
}
