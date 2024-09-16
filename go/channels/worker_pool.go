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
