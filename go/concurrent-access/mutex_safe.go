package main

import (
	"fmt"
	"sync"
)

var (
	counter int
	mu		sync.Mutex
	wg		sync.WaitGroup
)

func incrementCounter() {
	defer wg.Done()
	mu.Lock()
	counter++
	mu.Unlock()
}

func main() {
	numGoroutines := 100

	for i := 0; i < numGoroutines; i++ {
		wg.Add(1)
		go incrementCounter()
	}

	wg.Wait()
	fmt.Printf("Final counter value: %d\n", counter)
}
