#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define BUFFER_SIZE 1024

void download_file(int sock, const char *filename) {
  char buffer[BUFFER_SIZE];
  FILE *file = fopen(filename, "w");

  if (!file) {
    perror("Failed to open file");
    return;
  }

  while (1) {
    ssize_t bytes_received = recv(sock, buffer, sizeof(buffer), 0);
    if (bytes_received <= 0)
      break;
    fwrite(buffer, 1, bytes_received, file);
  }

  fclose(file);
  printf("Downloaded: %s\n", filename);
}

int main() {
  int sock = socket(AF_INET, SOCK_STREAM, 0);
  struct sockaddr_in server_address = {.sin_family = AF_INET,
                                       .sin_port = htons(21),
                                       .sin_addr.s_addr =
                                           inet_addr("127.0.0.1")};

  connect(sock, (struct sockaddr *)&server_address, sizeof(server_address));

  char response[BUFFER_SIZE];
  recv(sock, response, sizeof(response), 0);
  printf("Server Response: %s\n", response);

  const char *user = "USER anonymous\r\n";
  send(sock, user, strlen(user), 0);
  recv(sock, response, sizeof(response), 0);
  printf("Server Response: %s", response);

  const char *pass = "PASS anonymous\r\n";
  send(sock, pass, strlen(pass), 0);
  recv(sock, response, sizeof(response), 0);
  printf("Server Response: %s\n", response);

  const char *command = "RETR filename.txt\r\n";
  send(sock, command, strlen(command), 0);
  download_file(sock, "downloaded_file.txt");

  close(sock);
  return 0;
}
