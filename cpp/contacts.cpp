#include <iostream>
#include <map>
#include <string>

struct Contact {
	std::string name;
	std::string phoneNumber;
};

void addContract(std::map<std::string, Contact>& contacts, const std::string& name, const std::string& phoneNumber) {
	Contact newContact = {name, phoneNumber};
	contacts[name] = newContact;
	std::cout << "Contact added: " << name << " (" << phoneNumber << ")" << std::endl;
}
