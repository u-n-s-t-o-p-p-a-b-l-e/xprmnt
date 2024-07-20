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

func (list *CircularLinkedList) printList() {
	if list.head == nil {
		fmt.Println("List is empty")
		return
	}
	current := list.head
	for {
		fmt.Print(current.value, " -> ")
		current = current.next
		if current == list.head {
			break
		}
	}
	fmt.Println("head")
}

func main() {
	list := CircularLinkedList{}

	list.append(1)
	list.append(2)
	list.append(3)

	list.printList()
}
