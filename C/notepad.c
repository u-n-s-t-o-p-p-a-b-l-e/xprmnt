#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NOTE_LENGTH 256
#define FILENAME "notes.txt"

void add_note() {
	FILE *file = fopen(FILENAME, "a");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	char note[MAX_NOTE_LENGTH];
	printf("Enter your note: ");
	getchar();
	fgets(note, MAX_NOTE_LENGTH, stdin);
	note[strcspn(note, "\n")] = '\0';

	fprintf(file, "%s\n", note);
	fclose(file);

	printf("Note added successfully.\n");
}

void view_notes() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	char note[MAX_NOTE_LENGTH];
	printf("Notes:\n");

	while (fgets(note, MAX_NOTE_LENGTH, file) != NULL) {
		printf("%s", note);
	}
	fclose(file);
}

void delete_notes() {
	FILE *file = fopen(FILENAME, "w");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}
	fclose(file);

	printf("All notes deleted successfully.\n");
}

int main() {
	int choice;
	while (1) {
		printf("\nNotepad\n");
		printf("1. Add Note\n");
		printf("2. View Notes\n");
		printf("3. Delete All Notes\n");
		printf("4. Exit\n");
		printf("Enter your choice: ");
		scanf("%d", &choice);
	}
}

