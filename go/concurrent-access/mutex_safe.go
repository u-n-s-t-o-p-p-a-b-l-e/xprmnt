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
