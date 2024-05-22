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

	char tasks[MAX_TASK_LENGTH][MAX_TASK_LENGTH];
	int task_count = 0;

	while (fgets(tasks[task_count], MAX_TASK_LENGTH, file) != NULL) {
		tasks[task_count][strcspn(tasks[task_count], "\n")] = '\0';
		task_count++;
	}
	fclose(file);

	int task_number;
	printf("Enter the task number to delete: ");
	scanf("%d", &task_number);

	if (task_number < 1 || task_number > task_count) {
		printf("Invalid task number.\n");
		return;
	}

	file = fopen(FILENAME, "w");
	if (file == NULL) {
		perror("Unable to open file");
		return;
	}

	for (int i = 0; i < task_count; i++) {
		if (i != task_number -1) {
			fprintf(file, "%s\n", tasks[i]);
		}
	}
}
