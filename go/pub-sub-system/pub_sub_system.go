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

func NewPublisher() *Publisher {
	return &Publisher{
		subscribers: make(map[Subscriber]struct{}),
		addSub:		 make(chan Subscriber),
		removeSub:	 make(chan Subscriber),
		publish:	 make(chan Message),
	}
}
