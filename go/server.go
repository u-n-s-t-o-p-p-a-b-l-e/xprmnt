package main

import (
	"bufio"
	"fmt"
	"net"
)

func main() {
	listener, err := net.Listen("tcp", ":8080")
	if err != nil {
		fmt.Println("Error: ", err)
		return
		}
		defer listener.Close()

		fmt.Println("Server started. Listening on port 8080...")

		for {
			conn, err := listener.Accept()
			if err != nil {
				fmt.Println("Errror accepting connection:", err)
				continue
			}

			go handleConnection(conn)
		}
		
}
