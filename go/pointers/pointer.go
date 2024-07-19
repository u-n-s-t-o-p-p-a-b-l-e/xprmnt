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

func main() {
	a := 10
	b := w0

	fmt.Println("Befor swap:")
	fmt.Println("a =", a)
	fmt.Println("b =", b)

	swap(&a, &b)	
}
