#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <fcntl.h>

#define PORT 8080
#define BUFFER_SIZE 1024

void handle_client(int new_socket) {
	char buffer[BUFFER_SIZE] = {0};
	char *file_name = "received_file";
	int bytes_read;
	int file_fd;

	file_fd = open(file_name, 0_WRONLY | O_CREAT | O_TRUNC, 0664);
	if (file_fd < 0) {
		perror("File open error");
		close(new_socket);
		return;
	}




}
