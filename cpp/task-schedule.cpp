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
