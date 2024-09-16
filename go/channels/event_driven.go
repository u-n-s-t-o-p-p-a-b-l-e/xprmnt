package main

import (
	"fmt"
	"time"
)

type Event struct {
	Message string
}

func ProcessEvent(e Event) {
	fmt.Println("Processing event:", e.Message)
}

func main() {
	eventChan := make(chan Event, 5)
	done := make(chan bool)

	go func() {
		for {
			select {
			case event := <-eventChan:
				ProcessEvent(event)
			case <-done:
				return
			}
		}
	}()
}
