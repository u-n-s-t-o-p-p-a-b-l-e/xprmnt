package main

import "fmt"

type Node struct {
	value int
	next *Node
}

type LinkedList struct {
	head *Node
}
