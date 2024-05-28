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



