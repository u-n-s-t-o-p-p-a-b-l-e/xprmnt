package main

import (
	"fmt"
	"reflect"
)

type Person struct {
	Name string
	Age int
}

func main() {
	p := Person{"Alice", 30}

	t := reflect.TypeOf(p)
	v := reflect.ValueOf(p)
}
