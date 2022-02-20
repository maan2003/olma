use std::{cell::RefCell, mem, ops, rc::Rc};

use bumpalo::Bump;

pub struct B<T> {
    // borrows from the bump allocator
    val: T,
    bump: ViewBump,
}

#[derive(Clone)]
pub struct ViewBump(Rc<Bump>);

impl ops::Deref for ViewBump {
    type Target = Bump;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

thread_local! {
    static VIEW_BUMP: RefCell<Option<ViewBump>> = RefCell::new(None);
}

impl ViewBump {
    pub fn init() {
        VIEW_BUMP.with(|bump| {
            let b = &mut *bump.borrow_mut();
            match b {
                Some(bump) => match Rc::get_mut(&mut bump.0) {
                    Some(bump) => {
                        bump.reset();
                    }
                    None => {
                        eprintln!("Bump allocator is not unique, allocating new bump");
                        *b = Some(ViewBump(Rc::new(Bump::new())));
                    }
                },
                None => {
                    *b = Some(ViewBump(Rc::new(Bump::new())));
                }
            }
        });
    }

    pub fn current() -> ViewBump {
        match VIEW_BUMP.with(|bump| bump.borrow().clone()) {
            Some(b) => b,
            None => panic!("No bump allocator set"),
        }
    }

    pub fn reset() {
        VIEW_BUMP.with(|bump| {
            let b = &mut *bump.borrow_mut();
            match b {
                Some(bump) => match Rc::get_mut(&mut bump.0) {
                    Some(bump) => {
                        bump.reset();
                    }
                    None => {
                        eprintln!("Bump allocator is not unique, allocating new bump");
                        *b = Some(ViewBump(Rc::new(Bump::new())));
                    }
                },
                None => {
                    *b = Some(ViewBump(Rc::new(Bump::new())));
                }
            }
        });
    }
}

impl<T> B<T> {
    pub unsafe fn from_fn(f: impl FnOnce(&'static Bump) -> T) -> Self {
        let bump = ViewBump::current();
        Self::from_fn_in(bump, f)
    }

    pub unsafe fn from_fn_in(bump: ViewBump, f: impl FnOnce(&'static Bump) -> T) -> Self {
        let val = f(mem::transmute::<&'_ Bump, &'static Bump>(&*bump.0));
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

    pub unsafe fn from_raw(bump: ViewBump, ptr: *mut T) -> Self {
        B::from_fn_in(bump, move |_bump| bumpalo::boxed::Box::from_raw(ptr))
    }

    pub fn into_raw(this: Self) -> (ViewBump, *mut T) {
        (this.bump, bumpalo::boxed::Box::into_raw(this.val))
    }
}

#[macro_export]
macro_rules! vbox_dyn {
    ($v:expr, $t:ty) => {
        match $v {
            v => {
                let bump = $crate::view_bump::ViewBump::current();
                let value = bump.alloc(v) as &mut $t as *mut $t;
                unsafe { $crate::view_bump::VBox::from_raw(bump, value) }
            }
        }
    };
}
