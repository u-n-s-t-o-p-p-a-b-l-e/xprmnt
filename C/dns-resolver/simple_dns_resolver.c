#include <arpa/inet.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

void resolve_domain(const char *domain) {
  struct addrinfo hints, *res;
  struct sockaddr_in *addr;
  char ipstr[INET_ADDRSTRLEN];

  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;

  int status = getaddrinfo(domain, NULL, &hints, &res);
  if (status != 0) {
    fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(status));
    return;
  }

  addr = (struct sockaddr_in *) res->ai_addr;
  inet_ntop(res->ai_family, &addr->sin_addr, ipstr, sizeof(ipstr));
  printf("Resolved IP: %s\n", ipstr);

  freeaddrinfo(res);
}

int main() {
const char *domain = "www.example.com";
printf("Resolving domain: %s\n", domain);
resolve_domain(domain);
return 0;
}
