package main

import (
	"fmt"
	"time"
)

func Worker(id int, tasks <-chan int, results chan<- int) {
	for task := range tasks {
		fmt.Printf("Worker %d is processing task %d\n", id, task)
		time.Sleep(time.Second)
		results <- task * 2
	}
}

func main() {
	const numWorkers = 3
	const numTasks = 5

	tasks := make(chan int, numTasks)
	results := make(chan int, numTasks)

	for i := 1; i <= numWorkers; i++ {
		go Worker(i, tasks, results)
	}

	for i := 1; i <= numTasks; i++ {
		tasks <- i
	}
	close(tasks)

	for i := 1; i <= numTasks; i++ {
		result := <-results
		fmt.Printf("Result: %d\n", result)
	}
}
