package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"
)

const (
	address      = "localhost:8080"
	maxWorkers   = 1000
	bufferSize   = 4096
	maxIdleConns = 100
)
