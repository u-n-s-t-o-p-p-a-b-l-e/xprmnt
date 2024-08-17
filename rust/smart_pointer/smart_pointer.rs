use std::ops::Deref;

struct RcBox<T> {
    value: T,
    ref_count: usize,
}

pub struct MyRc<T> {
    ptr: *mut RcBox<T>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let box_ = Box::new(RcBox {
            value,
            ref_count: 1,
        });
        MyRc {
            ptr: Box::into_raw(box_),
        }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).ref_count += 1;
        }
        MyRc { ptr: self.ptr }
    }
}
