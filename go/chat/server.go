package main

import (
	"bufio"
	"fmt"
	"net"
	"strings"
)

var clients = make(map[net.Conn]string)
var messages = make(chan string)

fun handleConnection(conn net.Conn) {
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
}
