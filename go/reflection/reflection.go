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

	fmt.Printf("Type: %s\n", t.Name())
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		value := v.Field(i).Interface()
		fmt.Printf("%s: %v\n", field.Name, value)
	}

	pPointer := reflect.ValueOf(&p).Elem()
	nameField := pPointer.FieldByName("Name")
	if nameField.CanSet() {
		nameField.SetString("Bob")
	}

	fmt.Println("Modified struct:", p)
}
