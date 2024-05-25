package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	tasks := make([]string, 0)

	fmt.Println("\nSimple To-Do List Manager")
	for {
		fmt.Println("\n1. Add Task")
		fmt.Println("\n2. View Tasks")
		fmt.Println("\n3. Exit")
		fmt.Println("Choose an option: ")

		reader := bufio.NewReader(os.Stdin)
		option, _ := reader.ReadString('\n')
		option = strings.TrimSpace(option)

		switch option {
		case "1":
			fmt.Print("Enter task: ")
			task, _ := reeader.ReadString('\n')
			task = strings.TrimSpace(task)
			if task != "" {
				tasks = append(tasks, task)
				fmt.Println("Task added successfully")
			} else {
				fmt.Println("Task cannot be empty.")
			}
		case "2":
			if len(tasks) == 0 {
				fmt.Println("No tasks.")
			} else {
				fmt.Println("Tasks:")
				for i, task := range tasks {
					fmt.Printf("%dd. %s\n", i+1, task)
					
				}
				
			}
		case 3:
			fmt.Println("Exiting..")
			return
		default:
			fmt.Println("Invalid option. Please choose again.")
		}
	}
}


