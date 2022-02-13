#![feature(generic_associated_types)]
#![allow(clippy::needless_lifetimes)]

use std::mem;
use std::rc::Rc;
use std::{any::TypeId, mem::ManuallyDrop, ptr};

use bumpalo::Bump;

// SAFETY: build must only be called once and must be treated as if it is `self`
// caller must not drop `self`
unsafe trait View<'a>: 'a {
    fn type_id(&self) -> TypeId;
    fn build(&mut self) -> Box<dyn Widget<'a>>;
}

// VIEW describes a widget, widget can be built from a view
trait CustomView<'a>: 'a {
    // TODO(safety): this trait should be unsafe=
    fn type_id(&self) -> TypeId;
    fn build(self) -> Box<dyn Widget<'a>>;
}

trait CustomWidget {
    type View<'t>: CustomView<'t>;
    type This<'t>: CustomWidget + 't;

    // update the reference in widget from 'orig to 'new
    fn update<'orig, 'new>(this: Self::This<'orig>, view: Self::View<'new>) -> Self::This<'new>;
}

/// # Safety
///  - it should be safe to transmute the widget from 'a to 'b after call to `update`
unsafe trait Widget<'a>: 'a {
    /// update the references in widget from 'a to 'b in the widget
    ///
    /// # Safety
    ///  - caller needs to use ManuallyDrop to ensure view is not dropped by the caller
    ///  - *mut dyn View<'b> must be treated as if it was a `dyn View<'b>`
    unsafe fn update<'b>(&mut self, view: *mut dyn View<'b>);
}

struct WidgetWrap<'a, W: CustomWidget> {
    inner: <W as CustomWidget>::This<'a>,
}

unsafe impl<'a, C: CustomView<'a>> View<'a> for C {
    fn type_id(&self) -> TypeId {
        <C as CustomView>::type_id(self)
    }

    fn build(&mut self) -> Box<dyn Widget<'a>> {
        let this = self as *mut Self;
        // SAFETY: caller must ensure that the view is not dropped by the caller
        let this = unsafe { ptr::read(this) };
        this.build()
    }
}

impl<'a, W: CustomWidget> std::ops::Deref for WidgetWrap<'a, W> {
    type Target = <W as CustomWidget>::This<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, W: CustomWidget> std::ops::DerefMut for WidgetWrap<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl<'a, W> Widget<'a> for WidgetWrap<'a, W>
where
    W: CustomWidget + 'a,
{
    unsafe fn update<'b>(&mut self, view: *mut dyn View<'b>) {
        // SAFETY: view is valid pointer
        if TypeId::of::<W::View<'static>>() != (*view).type_id() {
            panic!("view type mismatch");
        }

        let view_ptr = view.cast::<W::View<'b>>();
        let view = ptr::read(view_ptr);
        // make segfaults very likely if caller tries to use view after the call to update
        ptr::write_bytes(view_ptr, 0xaa, 1);
        take_mut::take(&mut self.inner, |this| {
            let this = W::update(this, view);
            let mut this = ManuallyDrop::new(this);
            // mem::tranmute thinks W::This<'a> can of different size than W::This<'b>
            let this_ptr = &mut *this as *mut W::This<'b> as *mut u8 as *mut W::This<'a>;
            ptr::read(this_ptr)
        })
    }
}

struct AnyWidget<'a> {
    inner: Box<dyn Widget<'a>>,
}

impl<'a> AnyWidget<'a> {
    pub fn new<W: Widget<'a>>(inner: W) -> Self {
        AnyWidget {
            inner: Box::new(inner),
        }
    }

    pub fn update<'b>(mut self, view: AnyView<'b>) -> AnyWidget<'b> {
        let raw: *mut dyn View<'b> = Box::into_raw(view.inner);
        unsafe {
            // FIXME
            self.inner.update(mem::transmute(raw));
            // only the drop the allocated memory
            drop(Box::from_raw(raw as *mut ManuallyDrop<dyn View<'b>>));
            let inner: Box<dyn Widget<'b>> = mem::transmute(self.inner);
            AnyWidget { inner }
        }
    }
}

struct AnyView<'a> {
    inner: Box<dyn View<'a>>,
}

impl<'a> AnyView<'a> {
    pub fn new<W: View<'a>>(inner: W) -> Self {
        AnyView {
            inner: Box::new(inner),
        }
    }

    pub fn build(mut self) -> AnyWidget<'a> {
        let widget = self.inner.build();
        let ptr = Box::into_raw(self.inner);
        unsafe {
            // only drope the allocated memory
            // the build call will drop the view
            let _ = Box::from_raw(ptr as *mut ManuallyDrop<dyn View<'a>>);
        }
        AnyWidget { inner: widget }
    }
}

struct ListView<'a, T> {
    list: &'a [T],
    child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>,
}

struct List<'a, T> {
    list: &'a [T],
    // I surrender to the compiler, the lifetimes got too complicated
    // I will just use heap allocation
    child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>,
    children: Vec<AnyWidget<'a>>,
}

impl<'a, T> CustomView<'a> for ListView<'a, T>
where
    T: 'static,
{
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget<'a>> {
        Box::new(WidgetWrap::<List<_>> {
            inner: List {
                list: self.list,
                child: self.child,
                children: Vec::new(),
            },
        })
    }
}

impl<'a, T> CustomWidget for List<'a, T>
where
    T: 'static,
{
    type View<'t> = ListView<'t, T>;
    type This<'t> = List<'t, T>;

    fn update<'orig, 'new>(
        mut this: Self::This<'orig>,
        view: Self::View<'new>,
    ) -> Self::This<'new> {
        this.children.truncate(view.list.len());
        let mut it = view.list.iter();
        let mut children = this
            .children
            .into_iter()
            .map(|w| {
                let view = (view.child)(it.next().unwrap());
                w.update(view)
            })
            .collect::<Vec<_>>();

        for elems in it {
            let view = (view.child)(elems);
            let widget = view.build();
            children.push(widget);
        }

        List {
            list: view.list,
            child: view.child,
            children,
        }
    }
}

struct Lazy<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
    // has references into data
    inner: AnyWidget<'static>,
}

struct LazyView<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
}

impl<'a, T: 'static> CustomView<'a> for LazyView<T> {
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget<'a>> {
        // VIEW doesn't outline the data
        let view = unsafe { (self.builder)(&*(&*self.data as *const T)) };
        Box::new(WidgetWrap::<Lazy<_>> {
            inner: Lazy {
                inner: view.build(),
                data: self.data,
                builder: self.builder,
            },
        })
    }
}

impl<'a, T: 'static> CustomWidget for Lazy<T> {
    type View<'t> = LazyView<T>;
    type This<'t> = Lazy<T>;

    fn update<'orig, 'new>(this: Self::This<'orig>, view: Self::View<'new>) -> Self::This<'new> {
        if Rc::ptr_eq(&this.data, &view.data) && this.builder as usize == view.builder as usize {
            this
        } else {
            // VIEW doesn't outline the data
            let inner_view = unsafe { (view.builder)(&*(&*view.data as *const T)) };
            Lazy {
                data: view.data,
                builder: view.builder,
                inner: this.inner.update(inner_view),
            }
        }
    }
}
