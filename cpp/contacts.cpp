#include <iostream>
#include <map>
#include <string>

struct Contact {
	std::string name;
	std::string phoneNumber;
};

void addContact(std::map<std::string, Contact>& contacts, const std::string& name, const std::string& phoneNumber) {
	Contact newContact = {name, phoneNumber};
	contacts[name] = newContact;
	std::cout << "Contact added: " << name << " (" << phoneNumber << ")" << std::endl;
}

void viewContacts(const std::map<std::string, Contact>& contacts) {
	std::cout << "Contacts:\n";
	for (const auto& pair: contacts) {
		std::cout << "Name: " << pair.second.name << ", Phone Number: " << pair.second.phoneNumber << std::endl;
	}
}

void searchContacts(const std::map<std::string, Contact>& contacts, const std::string& name) {
	auto it = contacts.find(name);
	if (it != contacts.end()) {
		std::cout << "Name: " << it->second.name << ", Phone Number: " << it->second.phoneNumber << std::endl;
	} else {
		std::cout << "Contact not found." << std::endl;
	}
}
