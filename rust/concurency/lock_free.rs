use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::mem::MaybeUninit;

struct Node<T> {
    data: MaybeUninit<T>,
    next: AtomitPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Node {
            data: MaybeUninit::uninit(),
                  next: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node::new()));
        Queue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, value: T) {
        let mut new_node = Box::new(Node::new());
        new_node.data.write(value);
        let new_node_ptr = Box::into_raw(new_node);

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {
                    match unsafe { (*tail).next_compare_exchange(
                        ptr::null_mut(),
                        new_node_ptr,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) } {
                        Ok(_) => {
                            let _ = self.tail.compare_exchange(
                                tail,
                                new_node_ptr,
                                Ordering::Release,
                                Ordering::Relaxed,
                            );
                            return;
                        }

                        Err(_) => continue,
                    }
                } else {
                    let _ = self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if next.is_null() {
                        return None;
                    }

                    let _ = self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                } else {
                    let value = unsafe { ptr::read((*next).data.as_ptr()) };
                    match self.head.compare_exchange(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            unsafe { Box::form_raw(head); }
                            return Some(value);
                        }
                        Err(_) => continute,
                    }
                }
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        let head = self.head.load(Ordering::Relaxed);
        unsafe { Box::from_raw(head); }
    }
} 

fn main() {
    let queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
}
