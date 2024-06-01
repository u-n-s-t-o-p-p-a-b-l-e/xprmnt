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

void viewTasks(const std::vector<Task>& tasks) {
	std::cout << "Tasks:\n";
	for (size_t i = 0; i < tasks.size(); ++i) {
		std::cout << i + 1 << ". ";
		if (tasks[i].completed) {
			std::cout << "[X] ";
		} else {
			std::cout << "[  ] ";
		}
		std::cout << tasks[i].description << " (Due: " << ctime(&tasks[i].dueDate) << ")";
	}
}

void remindUpcomingTasks(const std::vector<Task>& tasks) {
	time_t currentTime = time(nullptr);
	std::cout << "Upcoming tasks:\n";
	for (const auto& task : tasks) {
		if (!task.completed && task.dueDate > currentTime && task.dueDate <= currentTime + 86400) {
			std::cout << task.description << " (Due: " << ctime(&task.dueDate) << ")";
		}
	}
}

int main() {
	while (true) {
		std::cout << "\nTask Scheduler\n";
		std::cout << "1. Add Task\n";
		std::cout << "2. Complete Task\n";
		std::cout << "3. View Tasks\n";
		std::cout << "4. Remind Upcoming Tasks\n";
		std::cout << "5. Exit\n";
		std::cout << "Enter your choice: ";

		int choice;
		std::cin >> choice;

		switch (choice) {
			case 1: {
						std::cin.ignore();
						std::string description;
						std::cout << "Enter task description: ";
						std::getline(std::cin, description);
						time_t dueDate;
						std::cout << "enter due date and time (YYYY-MM-DD HH::MM): ";
						std::cin >> std::get_time(&dueDate, "%Y-%m-%d %H:%M");
						addTask(tasks, description, dueDate);;;
						break;
					}
		}
	}
}
