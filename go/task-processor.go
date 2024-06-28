package main

import (
	"bufio"
	"fmt"
	"os"
	"sync"
	"time"
)

type Task struct {
	Name		string
	Duration	time.Duration
}

func worker(id int, tasks <-chan Task, wg *sync.WaitGroup) {
	defer wg.Done()
	for task := range tasks{
		fmt.Printf("Worker %d: Processing task %s\n", id, task.Name)
		time.sleep(task.Duration)
		fmt.Printf("Worker %d: Finished task %s\n", id, task.Name)
	}
}

func main() {
	const numWorkers = 3
	tasks := make(chan Task)
	var wg sync.WaitGroup

	for i := 1; i <= numWorkers; i++ {
		wg.Add(1)
		go worker(i, tasks, &wg)
	}

	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("Enter tsks in ther format 'name duration(s)' (e.g., 'task1 2s').Enter 'exit' to finish.")

	for scanner.Scan() {
		input := scanner.Text()
		if input == "exit" {
			break
		}

		var name string
		var duration time.Duration
		_, err :=  fmt.Sscanf(input, "%s %ds", &name, &duration)
		if err != nil {
			fmt.Println("Invalid input. Please enter in the format 'name duration(s)' (e.g., 'task1 2s').")
			continue
		}

		tasks <- Task{Name: name, Duration: duration * time.Second}
	}

	close(tasks)
	wg.Wait()
	fmt.Println("All tasks processed. Exiting")
	
	
}
