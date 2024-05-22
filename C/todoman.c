#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TASK_LENGTH 256
#define FILENAME "todo.txt"

void add_task() {
	FILE *file = fopen(FILENAME, "a");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	char task[MAX_TASK_LENGTH];
	printf("Enter the task: ");
	getchar();
	fgets(task, MAX_TASK_LENGTH, stdin);
	task[strcspn(task, "\n")] = '\0';

	fprintf(file, "%s\n", task);
	fclose(file);

	printf("Task added successfully.\n");
}

void view_tasks() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	char task[MAX_TASK_LENGTH];
	int task_number = 1;

	printf("To-Do List:\n");
	while (fgets(task, MAX_TASK_LENGTH, file) != NULL) {
		printf("%d. %s", task_number++, task);
	}
	fclose(file);
}

void delete_task() {
	FILE *file = fopen(FILENAME, "r");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}
}
