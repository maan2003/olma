use std::{cell::RefCell, mem, ops, rc::Rc};

use bumpalo::Bump;

pub struct B<T> {
    // borrows from the bump allocator
    val: T,
    bump: Rc<Bump>,
}

thread_local! {
    static BUMP: RefCell<Option<Rc<Bump>>> = RefCell::new(None);
}

pub fn current() -> Rc<Bump> {
    match BUMP.with(|bump| bump.borrow().clone()) {
        Some(b) => b,
        None => panic!("No bump allocator set"),
    }
}

pub fn new() {
    BUMP.with(|bump| {
        let b = &mut *bump.borrow_mut();
        match b {
            Some(bump) => match Rc::get_mut(bump) {
                Some(bump) => {
                    bump.reset();
                }
                None => {
                    eprintln!("Bump allocator is not unique, allocating new bump");
                    *b = Some(Rc::new(Bump::new()));
                }
            },
            None => {
                *b = Some(Rc::new(Bump::new()));
            }
        }
    });
}

pub fn with_mut<R>(f: impl FnOnce(&mut Bump) -> R) -> Option<R> {
    BUMP.with(|bump| {
        let b = &mut *bump.borrow_mut();
        match b {
            Some(bump) => match Rc::get_mut(bump) {
                Some(bump) => Some(f(bump)),
                None => {
                    eprintln!("Bump allocator is not unique");
                    None
                }
            },
            None => panic!("No bump allocator set"),
        }
    })
}

/// Reset the bump allocator if it is unique
pub fn reset() {
    self::with_mut(|bump| {
        bump.reset();
    });
}

impl<T> B<T> {
    pub unsafe fn from_fn(f: impl FnOnce(&'static Bump) -> T) -> Self {
        let bump = current();
        Self::from_fn_in(bump, f)
    }

    pub unsafe fn from_fn_in(bump: Rc<Bump>, f: impl FnOnce(&'static Bump) -> T) -> Self {
        let val = f(mem::transmute::<&'_ Bump, &'static Bump>(&*bump));
        B { val, bump }
    }

    pub fn forget(this: Self) {
        mem::forget(this.val);
    }
}

impl<T> ops::Deref for B<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T> ops::DerefMut for B<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.val
    }
}

pub type VVec<'a, T> = B<bumpalo::collections::Vec<'a, T>>;
pub type VBox<'a, T> = B<bumpalo::boxed::Box<'a, T>>;

impl<'a, T> VVec<'a, T> {
    pub fn new() -> Self {
        unsafe { B::from_fn(|bump| bumpalo::collections::Vec::new_in(bump)) }
    }
    pub fn with_capacity(cap: usize) -> Self {
        unsafe { B::from_fn(|bump| bumpalo::collections::Vec::with_capacity_in(cap, bump)) }
    }
}

impl<'a, T> IntoIterator for VVec<'a, T> {
    type Item = T;
    type IntoIter = bumpalo::collections::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.val.into_iter()
    }
}

impl<'a, T: ?Sized> VBox<'a, T> {
    pub fn new(value: T) -> Self
    where
        T: Sized,
    {
        unsafe { B::from_fn(move |bump| bumpalo::boxed::Box::new_in(value, bump)) }
    }

    pub unsafe fn from_raw(bump: Rc<Bump>, ptr: *mut T) -> Self {
        B::from_fn_in(bump, move |bump| bumpalo::boxed::Box::from_raw(ptr))
    }

    pub fn into_raw(this: Self) -> (Rc<Bump>, *mut T) {
        (this.bump, bumpalo::boxed::Box::into_raw(this.val))
    }
}
