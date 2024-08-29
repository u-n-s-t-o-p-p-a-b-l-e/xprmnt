package main

import (
	"fmt"
	"time"
)

type Task struct {
	id int
}

func worker(id int, tasks <-chan Task, results chan<- int) {
	for task := range tasks {
		fmt.Printf("Worker %d started task %d\n", id, task.id)
		time.Sleep(1 * time.Second)
		fmt.Printf("worker %d finished task %d\n", id, task.id)
		results <- task.id
	}
}

func main() {
	numWorkers := 3
	numTasks := 5

	tasks := make(chan Task, numTasks)
	results := make(chan int, numTasks)

	for w := 1; w <= numWorkers; w++ {
		go worker(w, tasks, results)
	}

	for t := 1; t <= numTasks; t++ {
		tasks <- Task{id: t}
	}
	close(tasks)

	for t := 1; t <= numTasks; t++ {
		<-results
	}

	fmt.Println("All tasks completed")
}

