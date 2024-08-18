#[allow(unused_imports)]
use std::ptr;

struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        let ptr = Box::into_raw(Box::new(value));
        MyBox { ptr }
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

#[allow(unused_must_use)]
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
        println!("Memory has been deallocated!");
    }
}

fn main() {
    let x = MyBox::new(42);
    println!("MyBox contains: {}", *x);
}
