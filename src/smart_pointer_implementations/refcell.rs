use crate::smart_pointer_implementations::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefState {
    Exclusive,
    Shared(usize),
    NotShared,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    pub state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::NotShared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::NotShared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::NotShared => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
            RefState::Shared(_) => None,
            RefState::Exclusive => None,
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::NotShared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::NotShared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::NotShared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::NotShared);
            }
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn simple_count() {
        let initial = RefCell::new(String::from("Contents"));
        let _a = initial.borrow();
        let _b = initial.borrow();
        {
            let _c = initial.borrow();
            let state = initial.state.get();
            assert_eq!(state, RefState::Shared(3));
        }
        let state = initial.state.get();
        assert_eq!(state, RefState::Shared(2));
    }
}
