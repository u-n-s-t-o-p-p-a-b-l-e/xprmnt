package main

import (
	"bufio"
	"fmt"
	"net"
	"strings"
)

var clients = make(map[net.Conn]string)
var messages = make(chan string)

func handleConnection(conn net.Conn) {
	defer conn.Close()

	clients[conn] = conn.RemoteAddr().String()

	messages <- fmt.Sprintf("%s joined the chat", clients[conn])

	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		text := scanner.Text()
		if strings.TrimSpace(text) == "QUIT" {
			break
		}
		messages <- fmt.Sprintf("%s: %s", clients[conn], text)
	}


	delete(clients, conn)

	messages <- fmt.Sprintf("%s left the chat", clients[conn])

}

func broadcastMessages() {
	for {
		msg := <-messages
		for conn := range clients {
			fmt.Fprintln(conn, msg)
		}
	}
}

func main() {
	listener, err := net.Listen("tcp", ":8080")
	if err != nil {
		fmt.Println("Error starting server:", err)
		return
	}
	defer listener.Close()

	go broadcastMessages()

	fmt.Println("Chat server started on port 8080")
	
}
