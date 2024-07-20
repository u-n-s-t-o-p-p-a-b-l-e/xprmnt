package main

import "fmt"

type Node struct {
	value int
	next *Node
}

type CircularLinkedList struct {
	head *Node
}
