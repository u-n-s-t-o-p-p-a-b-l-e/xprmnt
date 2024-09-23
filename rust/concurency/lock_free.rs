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
}
