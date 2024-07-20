package main

import "fmt"

type Node struct {
	value int
	next *Node
}

type LinkedList struct {
	head *Node
}


func (list *LinkedList) append(value int) {
	newNode := &Node{value: value}
	if list.head == nil {
		list.head = newNode
	} else {
		current := list.head
		for current.next != nil {
			current = current.next
		}
		current.next = newNode
	}
}

func (list *LinkedList) printList() {
	current := list.head
	for current != nil {
		fmt.Print(current.value, " -> ")
		current = current.next
	}
}
