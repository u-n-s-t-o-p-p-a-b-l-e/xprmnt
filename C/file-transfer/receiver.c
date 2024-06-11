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

int main() {
	int sock = 0;
	struct sockaddr_in serv addr;
	const char *file_name = "file_to_send";

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

	if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) <0) {
		perror("Connection Failed");
		return -1;
	}

	send_file(sock, file_name);

	close(sock);

	return 0;
}
