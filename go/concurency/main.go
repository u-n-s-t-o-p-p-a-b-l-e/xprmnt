package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"os/signal"
	"runtime"
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
		conn.SetWriteDeadline(time.Now().Add(5 * time.Minute))
		_, err = writer.Write(buf[:n])
		if err != nil {
			fmt.Println("Write error:", err)
			break
		}
		writer.Flush()
	}
}

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())

	listener, err := net.Listen("tcp", address)
	if err != nil {
		fmt.Println("Error starting server:", err)
		return
	}
	defer listener.Close()

	var wg sync.WaitGroup
	connChan := make(chan net.Conn, maxWorkers)

	for i := 0; i < maxWorkers; i++ {
		go func() {
			for conn := range connChan {
				connectionHandler(conn, &wg)
			}
		}()
	}

	fmt.Println("Server started on", address)

	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-sigChan
		fmt.Println("\nShutting down server...")
		listener.Close()
		close(connChan)
	}()

	for {
		conn, err := listener.Accept()
		if err != nil {
			if opErr, ok := err.(*net.OpError); ok && opErr.Op == "accept" {
				break
			}
			fmt.Println("Error accepting connection:", err)
			continue
		}
		wg.Add(1)
		connChan <- conn
	}

	wg.Wait()
	fmt.Println("Server shut down gracefully.")
	
	
}



