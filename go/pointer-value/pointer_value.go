package main

import "fmt"

type Printer interface {
	Print()
}

type valueReceiver struct {
	value int
}
