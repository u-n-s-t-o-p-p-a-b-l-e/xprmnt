package main

import "fmt"

func swap(a, b *int) {
	temp := *a
	*a = *b
	*b = temp
}

func increment(n *int) {
	*n++

}

type Person struct {
	name string
	age int
}

func updateName(p *Person, newName string) {
	p.name = newName
}
