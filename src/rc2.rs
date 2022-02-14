use std::{cell::Cell, ops, ptr::NonNull};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Debug)]
struct Rc2Inner<T> {
    left: T,
    right: T,
    left_count: Cell<usize>,
    right_count: Cell<usize>,
}

impl<T> Rc2Inner<T> {
    /// Get a reference to the rc2 inner's right count.
    fn right_count(&self) -> &Cell<usize> {
        &self.right_count
    }
}

#[derive(Debug)]
struct Rc2<T: Clone> {
    ptr: NonNull<Rc2Inner<T>>,
    side: Side,
}

impl<T: Clone> Rc2<T> {
    fn new(left: T, right: T) -> Self {
        Rc2 {
            ptr: unsafe {
                NonNull::new_unchecked(Box::into_raw(Box::new(Rc2Inner {
                    left,
                    right,
                    left_count: Cell::new(1),
                    right_count: Cell::new(0),
                })))
            },
            side: Side::Left,
        }
    }

    pub fn swap(&mut self) {
        unsafe {
            self.dec();
            self.side = self.side.opposite();
            self.inc();
        }
    }

    pub fn current(&self) -> &T {
        unsafe {
            match self.side {
                Side::Left => &self.ptr.as_ref().left,
                Side::Right => &self.ptr.as_ref().right,
            }
        }
    }

    fn count(&self) -> usize {
        unsafe {
            match self.side {
                Side::Left => self.ptr.as_ref().left_count.get(),
                Side::Right => self.ptr.as_ref().right_count.get(),
            }
        }
    }

    unsafe fn set_count(&self, count: usize) {
        match self.side {
            Side::Left => self.ptr.as_ref().left_count.set(count),
            Side::Right => self.ptr.as_ref().right_count.set(count),
        }
    }

    unsafe fn inc(&self) {
        let count = self.count();
        self.set_count(count + 1);
    }

    unsafe fn dec(&self) {
        let count = self.count();
        self.set_count(count - 1);
    }

    pub fn is_unique(&self) -> bool {
        self.count() == 1
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            self.ptr.as_ref().left_count.get() == 0 && self.ptr.as_ref().right_count.get() == 0
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_unique() {
            unsafe {
                return match self.side {
                    Side::Left => Some(&mut self.ptr.as_mut().left),
                    Side::Right => Some(&mut self.ptr.as_mut().right),
                };
            }
        }
        self.swap();
        if self.is_unique() {
            let val = unsafe {
                match self.side {
                    Side::Left => &mut self.ptr.as_mut().left,
                    Side::Right => &mut self.ptr.as_mut().right,
                }
            };
            let other = unsafe {
                match self.side {
                    Side::Left => &self.ptr.as_ref().right,
                    Side::Right => &self.ptr.as_ref().left,
                }
            };
            val.clone_from(other);
            return Some(val);
        }
        None
    }
}

impl<T: Clone> Drop for Rc2<T> {
    fn drop(&mut self) {
        unsafe {
            self.dec();
            if self.is_empty() {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T: Clone> Clone for Rc2<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.inc();
        }
        Rc2 {
            ptr: self.ptr,
            side: self.side,
        }
    }
}

impl<T: Clone> ops::Deref for Rc2<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.current()
    }
}

impl<T: Clone> ops::DerefMut for Rc2<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut().unwrap()
    }
}

#[test]
fn check() {
    let a = Rc2::new(1, 1);
    let mut b = a.clone();
    *b += 1;
    assert_eq!(a.side, Side::Left);
    assert_eq!(b.side, Side::Right);
    assert_eq!(*a.current(), 1);
    assert_eq!(*b.current(), 2);

    drop(a);
    let mut a = b.clone();
    let c = b.clone();
    let d = b.clone();
    let e = b.clone();
    *a += 1;
    assert_eq!(a.side, Side::Left);
    assert_eq!(b.side, Side::Right);
    assert_eq!(*a.current(), 3);
    assert_eq!(*b.current(), 2);
    assert_eq!(c.side, Side::Right);
    assert_eq!(d.side, Side::Right);
    assert_eq!(e.side, Side::Right);
    assert_eq!(*c.current(), 2);
    assert_eq!(*d.current(), 2);
    assert_eq!(*e.current(), 2);
}
