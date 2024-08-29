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

func (p *Publisher) Subscribe() Subscriber {
	sub := make(Subscriber)
	p.addSub <- sub
	return sub
}

func (p *Publisher) Unsubscribe(sub Subscriber) {
	p.removeSub <- sub
	close(sub)
}

func (p *Publisher) Start() {
	go func() {
		for {
			select {
			case sub :=  <-p.addSub:
				p.subscribers[sub] = struct{}{}
			case sub := <-p.removeSub:
				delete(p.subscribers, sub)
			case msg := <-p.publish:
				for sub := range p.subscribers {
					sub <- msg
				}
			}
		}
	}()
}
