#include <iostream>
#include <vector>
#include <string>
#include <ctime>
#include <iomanip> // For std::get_time
#include <limits> // For std::numeric_limits

struct Task {
    std::string description;
    time_t dueDate;
    bool completed;
};

// Function to add a new task
void addTask(std::vector<Task>& tasks, const std::string& description, time_t dueDate) {
    Task newTask = {description, dueDate, false};
    tasks.push_back(newTask);
    std::cout << "Task added: " << description << std::endl;
}

// Function to mark a task as completed
void completeTask(std::vector<Task>& tasks, size_t index) {
    if (index < tasks.size()) {
        tasks[index].completed = true;
        std::cout << "Task marked as completed: " << tasks[index].description << std::endl;
    } else {
        std::cout << "Invalid task index." << std::endl;
    }
}

// Function to view all tasks
void viewTasks(const std::vector<Task>& tasks) {
    std::cout << "Tasks:\n";
    for (size_t i = 0; i < tasks.size(); ++i) {
        std::cout << i + 1 << ". ";
        if (tasks[i].completed) {
            std::cout << "[X] ";
        } else {
            std::cout << "[ ] ";
        }
        std::cout << tasks[i].description << " (Due: " << std::put_time(std::localtime(&tasks[i].dueDate), "%Y-%m-%d %H:%M") << ")\n";
    }
}

// Function to remind upcoming tasks
void remindUpcomingTasks(const std::vector<Task>& tasks) {
    time_t currentTime = time(nullptr);
    std::cout << "Upcoming tasks:\n";
    for (const auto& task : tasks) {
        if (!task.completed && task.dueDate > currentTime && task.dueDate <= currentTime + 86400) { // Within the next day
            std::cout << task.description << " (Due: " << std::put_time(std::localtime(&task.dueDate), "%Y-%m-%d %H:%M") << ")\n";
        }
    }
}

int main() {
    std::vector<Task> tasks;

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
                std::cin.ignore(); // Clear the input buffer
                std::string description;
                std::cout << "Enter task description: ";
                std::getline(std::cin, description);
                std::tm tm = {};
                std::cout << "Enter due date and time (YYYY-MM-DD HH:MM): ";
                std::cin >> std::get_time(&tm, "%Y-%m-%d %H:%M");
                if (std::cin.fail()) {
                    std::cin.clear();
                    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
                    std::cerr << "Invalid date format.\n";
                } else {
                    time_t dueDate = mktime(&tm);
                    addTask(tasks, description, dueDate);
                }
                break;
            }
            case 2: {
                size_t index;
                std::cout << "Enter task index to mark as completed: ";
                std::cin >> index;
                completeTask(tasks, index - 1); // Adjusting index to start from 1
                break;
            }
            case 3:
                viewTasks(tasks);
                break;
            case 4:
                remindUpcomingTasks(tasks);
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

