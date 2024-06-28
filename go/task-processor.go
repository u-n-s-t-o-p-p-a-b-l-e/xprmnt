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
}
