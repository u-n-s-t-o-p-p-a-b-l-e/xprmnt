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
	item.name[strcspn(item.name, "\n")] = '\0';

	printf("Enter the quantity: ");
	scanf("%d", &item.quantity);

	printf("Enter the price: ");
	scanf("%f", &item.price);

	fprintf(file, "%s %d %.2f\n", item.name, item.quantity, item.price);
	fclose(file);

	printf("Item added successfully");
}

void view_items() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Item item;
	printf("Inventory:\n");
	printf("%-20s %-10d %-10.2f\n", "Name", "Quantity", "Price");
	 while (fscanf(file, "%s %d %f", item.name, &item.quantity, &item.price) != EOF
		   ) {
		 printf("%-20s %-10d %-10.2f\n", item.name, item.quantity, item.price);
	 }
	 fclose(file);
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
	item_name[strcspn(item_name, "\n")] = '\0';

	file = fopen(FILENAME, "w");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	int found = 0;
	for (int i = 0; i < item_count; i++) {
		if (strcmp(items[i].name, item_name) != 0) {
			fprintf(file, "%s %d %.2f\n", items[i].name, items[i].quantity, items[i].price);
		} else {
			found = 1;
		}
	}
	fclose(file);

	if (found) {
		printf("Item deleted successfully\n");
	} else {
		printf("Item not found.\n");
	}
}

int main() {
	int choice;

	while (1) {
		printf("\nInventory Manager\n");
		printf("1. Add Item\n");
		printf("2. View Items\n");
		printf("3. Delete Item\n");
		printf("4. Exit\n");
		printf("5. Enter your choice: ");
		scanf("%d", &choice);

		switch (choice) {
			case 1:
				add_item();
				break;
			case 2:
				view_items();
				break;
			case 3:
				delete_item();
				break;
			case 4:
				printf("Exiting the program\n");
				exit(0);
			default:
				printf("Invalid choice. Please tyr again.\n");
		}
	}

	return 0;
}
