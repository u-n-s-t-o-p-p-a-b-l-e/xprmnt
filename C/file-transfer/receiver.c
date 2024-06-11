#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <fcntl.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void send_file(int sock, const char *file_name) {
	char buffer[BUFFER_SIZE] = {0};
	int bytes_read;
	int file_fd;

}
