#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080
#define BUFFER_SIZE 1024

int main() {
	int sock = 0;
	struct sockaddr_in serv_addr;
	char buffer[BUFFER_SIZE] = {0};
	const char *hello = "Hi from client";
}

if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
	printf("\n Socket creation error \n");
	return -1;
}

serv_addr.sin_family = AF_INET;
serv_addr.sin_port = htons(PORT);

if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
	printf("\nInvalid address/ Address not supported\n");
	return -1;
}
