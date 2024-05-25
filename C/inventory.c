#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_ITEM_NAME_LENGTH 100
#define MAX_ITEMS 100
#define FILENAME "inventory.txt"

typedef struct {
	char name[MAX_ITEM_NAME_LENGTH];
	int quantity;
	float price;
} Item;

void add_item() {
	FILE *file = fopen(FILENAME, "a");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}
}
