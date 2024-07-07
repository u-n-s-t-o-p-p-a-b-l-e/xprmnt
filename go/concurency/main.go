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

func connectionHandler(conn net.Conn, wg *sync.WaitGroup) {
	defer wg.Done()
	defer conn.Close()

	reader := bufio.NewReader(conn)
	writer := bufio.NewWriter(conn)
	buf := make([]byte, bufferSize)

	for {
		conn.SetReadDeadline(time.Now().Add(5 * time.Minute))
		n, err := reader.Read(buf)
		if err != nil {
			fmt.Println("Read error:", err)
			break
			
		}
	}

}


