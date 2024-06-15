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
