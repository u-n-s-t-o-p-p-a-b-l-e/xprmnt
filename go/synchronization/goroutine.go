package main

import (
	"fmt"
	"sync"
	"time"
)

func Worker(id int, wg *sync.WaitGroup) {
	defer wg.Done()

	
}
