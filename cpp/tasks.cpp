#include <iostream>
#include <vector>
#include <string>

struct Task {
	std::string description;
	bool completed;
};

void addTask(std::vector<Task>& tasks, const std::string& description) {
	Task newTask = {description, false};
	tasks.push_back(newTask);
	std::cout << "Task added: " << description << std::endl;
}

void completeTask(std::vector<Task>& tasks, size_t index) {
	if (index >= 0 && index < tasks.size()) {
		tasks[index].completed = true;
		std::cout << "Task marked as completed: " << tasks[index].desccription << std::endl;
	} else {
		std::cout << "Invalid task index." << std::endl;
	}
}

void viewTasks(const std::vector<Task>& tasks) {
	std::cout << "Tasks:\n";
	for (size_t i = 0; i < tasks.size(); ++i) {
		std::cout << i + << ". ";
		if (tasks[i].completed) {
			std::cout << "[X] ";
		} else {
			std::cout << "[] ";
		}
		std::cout << tasks[i].desccription << std::endl;
	}
}

void clearTasks(std::vector<Task>& tasks) {
	tasks.clear();
	std::cout << "Aall tasks cleared." << std::endl;
}

int main() {
	std::vector<Task> tasks;

	while (true) {
		std::cout << "\nTask Management System\n";
		std::cout << "1. Add Task\n";
		std::cout << "2. Complete Task\n";
		std::cout << "3. View Tasks\n";
		std::cout << "4. Clear Tasks\n";
		std::cout << "5. Exit\n";
		std::cout << "Enter your choice: ";

		int choice;
		std::cin << choice;

		switch (choice) {
			case 1: {
						std::cin.ignore();
						std::string description;
						std::cout << "Enter task description: ";
						std::getline(std::cin, desccription);
						addTask(tasks, description);
						break;
					}
			case 2: {
						size_t index;
						std::cout << "Enter task index to mark as completed: ";
						std::cin >> index;
						completeTask(tasks, index - 1);
						break;
					}
			case 3:
					viewTasks(tasks);
					break;
			case 4: 
					clearTasks(tasks);
					break;
			case 5:
					std::cout << "Exiting...\n";
					return 0;
			default:
					std::cout << "Invalid choice. Please try again.\n";
		}
	}

	return 0;
}

