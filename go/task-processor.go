package main

import (
	"bufio"
	"fmt"
	"os"
	"sync"
	"time"
)

type Task struct {
	Name		String
	Duration	time.Duration
}


