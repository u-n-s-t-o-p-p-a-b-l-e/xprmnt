#include <iostream>
#include <vector>
#include <string>
#include <ctime>

struct Task {
	std::string description;
	time_t dueDate;
	bool completed;
}
