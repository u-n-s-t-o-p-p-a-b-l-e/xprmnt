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

func (p *PointerReceiver) Print() {
	fmt.Println("PointerReceiver:", p.value)
}

func main() {
	v := ValueReceiver{value: 10}
	p := &PointerReceiver{value: 20}

	v.Print()
	p.Print()

	var printer Printer


}


