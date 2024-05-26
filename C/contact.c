#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAME_LENGTH 100
#define MAX_PHONE_LENGTH 15
#define FILENAME "contacts.txt"

typedef struct {
	char name[MAX_NAME_LENGTH];
	char phone[MAX_PHONE_LENGTH];
} contact;

void add_contact() {
	FILE *file = fopen(FILENAME, "a");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}
}

