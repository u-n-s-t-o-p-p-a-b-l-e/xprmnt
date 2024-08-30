package main

import "fmt"

type Printer interface {
	Print()
}

type ValueReceiver struct {
	value int
}

func (v ValueReceiver) Print() {
	fmt.Println("ValueReceiver:", v.value)
}

type PointerReceiver struct {
	value int
}

func (p *PointerReceiver) Print() {
	fmt.Println("PointerReceiver:", p.value)
}

func main() {
	v := ValueReceiver{value: 10}
	p := &PointerReceiver{value: 20}

	v.Print()
	p.Print()

	var printer Printer

	printer = v
	printer.Print()

	// The following line would cause a compile error:
	// printer = &v
	// printer.Print() // Does not work with pointer to value receiver


}


