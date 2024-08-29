package main

import "fmt"

type Shape interface {
	Area() float64
}

type Rectangle struct {
	width, height float64
}

type Circle struct {
	radius float64
}

func (r Rectangle) Area() float64 {
	return r.width * r.height
}

func (c Circle) Area() float64 {
	return 3.14 * c.radius * c.radius
}

func printArea(s Shape) {
	fmt.Printf("Area: %.2f\n", s.Area())
}

func main() {
	r := Rectangle{width: 10, height: 5}
	c := Circle{radius: 7}

	printArea(r)
	printArea(c)
}
