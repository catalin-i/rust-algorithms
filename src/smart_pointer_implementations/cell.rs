use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_references() {
        let cell = Cell::new(5);
        let a = cell.get();
        let b = cell.get();
        cell.set(7);
        assert_eq!(a, b);
        assert_eq!(a, 5)
    }
}
