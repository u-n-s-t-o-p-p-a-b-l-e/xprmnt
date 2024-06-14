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
}
