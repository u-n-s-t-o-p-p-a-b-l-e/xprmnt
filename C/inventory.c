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

	Item item;
	printf("Enter the item name: ");
	getchar();
	fgets(item.name, MAX_ITEM_NAME_LENGTH, stdin);
	item.name[strscpn(item.name, "\n")] = '\0';

	printf("Enter the quantity: ");
	scanf("%d", &item.quantity);

	printf("Enter the price: ");
	scanf("%f", &item.price);

	fprintf(file, "%s %d %.2f\n", item.name, item.quantity, item.price);
	flose(file);

	printf("Item added successfully");
}
