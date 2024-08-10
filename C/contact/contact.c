#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAME_LENGTH 100
#define MAX_PHONE_LENGTH 15
#define FILENAME "contacts.txt"

typedef struct {
	char name[MAX_NAME_LENGTH];
	char phone[MAX_PHONE_LENGTH];
} Contact;

void add_contact() {
	FILE *file = fopen(FILENAME, "a");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Contact contact;
	printf("Enter the contact name: ");
	getchar();
	fgets(contact.name, MAX_NAME_LENGTH, stdin);
	contact.name[strcspn(contact.name, "\n")] = '\0';

	printf("Enter the phone number: ");
	fgets(contact.phone, MAX_NAME_LENGTH, stdin);
	contact.phone[strcspn(contact.phone, "\n")] = '\0';

	fprintf(file, "%s %s\n", contact.name, contact.phone);
	fclose(file);

	printf("Contact added successfully.\n");
}

void view_contacts() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Contact contact;
	printf("Contacts:\n");
	printf("%-20s %-15s\n", "Name", "Phone");

	while (fscanf(file, "%s %s", contact.name, contact.phone) != EOF) {
		printf("%-20s %-15s\n", contact.name, contact.phone);
	}
	fclose(file);
}

void delete_contact() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Contact contacts[MAX_NAME_LENGTH];
	int contact_count = 0;

	while (fscanf(file, "%s %s", contacts[contact_count].name, contacts[contact_count].phone) != EOF) {
		contact_count++;
	}
	fclose(file);

	char contact_name[MAX_NAME_LENGTH];
	printf("Enter the name of the contact to delete: ");
	getchar();
	fgets(contact_name, MAX_NAME_LENGTH, stdin);
	contact_name[strcspn(contact_name, "\n")] = '\0';

	file = fopen(FILENAME, "w");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	int found = 0;
	for (int i = 0; i < contact_count; i++) {
		if (strcmp(contacts[i].name, contact_name) != 0) {
			fprintf(file, "%s %s\n", contacts[i].name, contacts[i].phone);
		} else {
			found = 1;
		}
	}
	fclose(file);

	if (found) {
		printf("Contact deleted successfully.\n");
	} else {
		printf("Contact not found.\n");
	}
}

int main() {
	int choice;

	while (1) {
		printf("\nContact Book\n");
		printf("1. Add Contact\n");
		printf("2. View Contacts\n");
		printf("3. Delete Contacts\n");
		printf("4. Exit\n");
		printf("Enter your choice: ");
		scanf("%d", &choice);

		switch (choice) {
			case 1:
				add_contact();
				break;
			case 2:
				view_contacts();
				break;
			case 3:
				delete_contact();
				break;
			case 4:
				printf("Exiting the program.\n");
				exit(0);
			default: 
				printf("Invalid choice. Please try again.\n");
		}
	}

	return 0;
}
