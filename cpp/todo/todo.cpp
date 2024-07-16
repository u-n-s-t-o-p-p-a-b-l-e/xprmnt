#include <iostream>
#include <vector>
#include <string>

struct Task {
	std::string description;
	bool completed;
};

void addTask(std::vector<Task> &tasks, const std::string &description) {
	Task newTask = {description, false};
	tasks.push_back(newTask);
	std::cout << "Task added: " << description <<std::endl;
}

void viewTasks(const std::vector<Task> &tasks) {
	if (tasks.empty()) {
		std::cout << "No tasks available." << std::endl;
		return;
	}

	std::cout << "Tasks: " << std::endl;
	for (size_t i = 0; i < tasks.size(); ++i) {
		std::cout << i + 1 << ". ";
		if (tasks[i].completed) {
			std::cout << "[X] ";
		} else {
			std::cout << "[] ";
		}
		std::cout << tasks[i].description << std::endl;
	}
}

void markTaskAsCompleted(std::vector<Task> &tasks, size_t index) {
	if (index >= 1 && index <= tasks.size()) {
		tasks[index -1].completed = true;
		std::cout << "Task marked as completed." << std::endl;
	} else {
		std::cout << "Invalid task number. " << std::endl;
	}
}

int main() {
	std::vector<Task> tasks;

	while (true) {
		std::cout << "\nTodo List Application\n";
		std::cout << "1. Add Task\n";
		std::cout << "2. View Tasks\n";
		std::cout << "3. Mark Task as Completed\n";
		std::cout << "4. Exit\n";
		std::cout << "5. Enter your choice: \n";

		int choice;
		std::cin >> choice;

		switch (choice) {
			case 1: {
						std::cin.ignore();
						std::string description;
						std::cout << "Enter task description: ";
						std::getline(std::cin, description);
						addTask(tasks, description);
						break;
					}
			case 2: 
					viewTasks(tasks);
					break;
			case 3:{
					   size_t index;
					   std::cout << "Enter the task number to mark as completed: ";
					   std::cin >> index;
					   markTaskAsCompleted(tasks, index);
					   break;
				   }
			case 4:
				   std::cout << "Exiting..\n";
				   return 0;
			default:
				   std::cout << "Invalid choice. Please try again.\n";

		}

	}

	return 0;
}

