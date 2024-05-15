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
