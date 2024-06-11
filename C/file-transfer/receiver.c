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

	file_fd = open(file_name, O_RDONLY);
	if (file_fd < 0) {
		perror("File open error");
		return;
	}

	while ((bytes_read = read(file_fd, buffer, BUFFER_SIZE)) > 0) {
		send(sock, buffer, bytes_read, 0);
	}

	printf("File sent succesfully\n");
	close(file_fd);
}
