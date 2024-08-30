package main

import "fmt"

type Printer interface {
	Print()
}

type valueReceiver struct {
	value int
}

func (v valueReceiver) Print() {
	fmt.Println("ValueReceiver:", v.value)
}

type PointerReceiver struct {
	value int
}
