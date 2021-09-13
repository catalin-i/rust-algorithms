use crate::smart_pointer_implementations::cell::Cell;
use std::ptr::NonNull;
pub struct RcShared<T> {
    value: T,
    count: Cell<usize>,
}

pub struct Rc<T> {
    shared: NonNull<RcShared<T>>,
}

impl<T> Rc<T> {
    pub fn new(val: T) -> Self {
        let shared = Box::new(RcShared {
            value: val,
            count: Cell::new(1),
        });
        Rc {
            shared: unsafe { NonNull::new_unchecked(Box::into_raw(shared)) },
        }
    }
    pub fn get_count(&self) -> usize {
        unsafe { self.shared.as_ref().count.get() }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let shared_ref = unsafe { self.shared.as_ref() };
        let num = shared_ref.count.get();
        shared_ref.count.set(num + 1);
        Rc {
            shared: self.shared,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.shared.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let shared = unsafe { self.shared.as_ref() };
        let count = shared.count.get();

        if count == 1 {
            let _ = unsafe { Box::from_raw(self.shared.as_mut()) };
        } else {
            shared.count.set(count - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_clones_and_drop() {
        let val = Rc::new(String::from("Test me!"));
        let clone1 = Rc::clone(&val);
        let clone2 = Rc::clone(&val);
        drop(val);
        assert_eq!(2, clone1.get_count());
        drop(clone1);
        assert_eq!(1, clone2.get_count());
    }
}
