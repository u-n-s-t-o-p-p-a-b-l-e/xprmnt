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

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).value }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).ref_count -= 1;
            if (*self.ptr).ref_count == 0 {
                let _ = Box::from_raw(self.ptr);
            }
        }
    }
}
