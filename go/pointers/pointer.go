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
	b := 20

	fmt.Println("Befor swap:")
	fmt.Println("a =", a)
	fmt.Println("b =", b)

	swap(&a, &b)	

	fmt.Println("After swap:")
	fmt.Println("a =", a)
	fmt.Println("b =", b)

	num := 5
	fmt.Println("\nBefore increment:", num)
	increment(&num)
	fmt.Println("After increment:", num)

	person := Person{name: "Alice", age: 30}
	fmt.Println("\nBefore name update:", person)
	updateName(&person, "Bob")
	fmt.Println("After name update:", person)

	ptr := &num
	ptrToPtr := &ptr
	fmt.Println("\nValue of num:", num)
	fmt.Println("Value of ptr:", *ptr)
	fmt.Println("Value of ptrToPtr:", **ptrToPtr)
}
