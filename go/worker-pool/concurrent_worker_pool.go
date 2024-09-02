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
		results <- task.IDD
	}
}
