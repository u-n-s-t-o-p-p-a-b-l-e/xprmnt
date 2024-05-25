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
	fclose(file);

	printf("Item added successfully");
}

void delete_item() {
	FILE *file = fopen(FILENAME, "r");;
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Item items[MAX_ITEMS];
	int item_count = 0;

	while (fscanf(file, "%s %d %f", items[item_count].name, &items[item_count].quantity, &items[item_count].price) != EOF) {
		item_count++;
	}
	fclose(file);

	char item_name[MAX_ITEM_NAME_LENGTH];
	printf("Enter the name of the item to delete: ");
	getchar();
	fgets(item_name, MAX_ITEM_NAME_LENGTH, stdin);
	item_name[strscpn(item_name, "\n")] = '\0';

	file = fopen(FILENAME, "w");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}
}

