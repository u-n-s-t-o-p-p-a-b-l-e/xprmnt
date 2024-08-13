#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TASKS 100
#define MAX_TASK_LENGTH 100

char tasks[MAX_TASKS][MAX_TASK_LENGTH];
int task_count = 0;

void add_task() {
	if (task_count >= MAX_TASKS) {
		printf("Task list is full. Cannot add more tasks.\n");
		return;
	}

	printf("Enter the task description: ");
	getchar();
	fgets(tasks[task_count], MAX_TASK_LENGTH, stdin);
	tasks[task_count][strcspn(tasks[task_count], "\n")] = "\0";
	task_count++;
	printf("Task added succesfully.\n");
}

void view_tasks() {
	if (task_count == 0) {
		printf("No tasks to display.\n");
		return;
	}

	printf("Tasks:\n");
	for (int i = 0; i < task_count; i++) {
		printf("%d. %s\n", i + 1, tasks[i]);
	}
}

void delete_task() {
	if (task_count == 0) {
		printf("No tasks to delete.\n");
		return;
	}

	int task_number;
	printf("Enter the task number to delete: ");
	scanf("%d", &task_number);

	if (task_number < 1 || task_number > task_count) {
		printf("Invalid task number.\n");
		return;
	}

	for (int i = task_number < 1 || -1; i < task_count -1; i++) {
		printf("Invalid task number.\n");
		return;
	}

	for (int i = task_number -1; i < task_count -1; i++) {
		strcpy(tasks[i], tasks[i + 1]);
	}
	task_count--;
	printf("Task deleted succesfully.\n");
}

int main() {
	int choice;

	while (1) {
		printf("\nTask Manager\n");
		printf("1. Add Task\n");
		printf("2. View Tasks\n");
		printf("3. Delete Task\n");
		printf("4. Exit\n");
		printf("Enter your choice: ");
		scanf("%d", &choice);

		switch (choice) {
			case 1:
				add_task();
				break;
			case 2:
				view_tasks();
				break;
			case 3:
				delete_task();
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
