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
	FILE *file == fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	Contact contacts[MAX_NAME_LENGTH];
	int contact_count = 0;

	while (fscanf(file, "%s %s", contacts[contact_count].name, contacts[contacts_count].phone) != EOF) {
		contact_count++;
	}
	fclose(file);
}

