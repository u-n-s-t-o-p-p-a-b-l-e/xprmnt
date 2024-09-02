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


