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

