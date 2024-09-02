package main

import (
	"fmt"
	"sync"
	"time"
)

type Task struct {
	ID	  int
	Delay time.Duration
}

func Worker(id int, tasks <-chan Task, results chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()

	for task := range tasks {
		fmt.Printf("Worker %d started task %d\n", id, task.ID)
		time.Sleep(task.Delay)
		fmt.Printf("Workeer %d finished task %d\n", id, task.ID)
		results <- task.ID
	}
}

func createWorkerPool(numWorkers int, tasks <-chan Task, results chan<- int) {
	var wg sync.WaitGroup

	for i := 1; i <= numWorkers; i++ {
		wg.Add(1)
		go Worker(i, tasks, results, &wg)
	}
}
