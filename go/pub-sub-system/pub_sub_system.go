package main

import (
	"fmt"
	"time"
)

type Message struct {
	Content string
}

type Subscriber chan Message
type Publisher struct {
	subscribers map[Subscriber]struct{}
	addSub		chan Subscriber
	removeSub	chan Subscriber
	publish		chan Message
}
