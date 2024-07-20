package main

import (
	"encoding"
	"fmt"
)

type Node struct {
	value int
	next *Node
}

type CircularLinkedList struct {
	head *Node
}

func (list *CircularLinkedList) append(value int) {
	newNode := &Node{value: value}
	if list.head == nil {
		list.head = newNode
		newNode.next = list.head
	} else {
		current := list.head
		for current.next != list.head {
			current = current.next
		}
		current.next = newNode
		newNode.next = list.head
	}
}
