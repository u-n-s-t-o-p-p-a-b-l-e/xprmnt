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

	while ((bytes_read = read(new_socket, buffer, BUFFER_SIZE)) > 0) {
		write(file_fd, buffer, bytes_read);
	}

	printf("File received succesfully\n");
	close(file_fd);
	close(new_socket);
}

int main() {
	int server_fd, new_socket;
	struct sockaddr_in address;
	int addrlen = sizeof(address);

	if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
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

	printf("Server listening on port %d\n", PORT);
}
