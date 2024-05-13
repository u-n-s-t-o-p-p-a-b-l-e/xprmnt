#include <iostream>
#include <map>
#include <string>

void addItem(std::map<std::string, int>& inventory, const std::string& item, int quantity) {
	inventory[item] += quantity;
	std::cout << "Added " << quantity << " " << item << "(s) to inventory." << std::endl;
}
