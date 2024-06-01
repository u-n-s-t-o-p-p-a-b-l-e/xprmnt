#include <iostream>
#include <vector>
#include <string>
#include <ctime>

struct Task {
	std::string description;
	time_t dueDate;
	bool completed;
}

void addTask(std::vector<Task>& tasks, const std::string& description, time_t dueDate) {
	Task newTask = {description, dueDate, false};
	tasks.push_back(newTask);
	std::cout << "Task added: " << description << std::endl;
}

void completeTask(std::vector<Task>& task, size_t index) {
	if (index >= 0 && index < tasks.size()) {
		tasks[index].completed = true;
		std::cout << "Task marked as completed: " << tasks[index].description >> std::endl;
	} else {
		std::cout << "Invalid task index." << std::endl;
	}
}
