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
