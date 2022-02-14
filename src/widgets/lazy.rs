use crate::core::*;

use std::any::TypeId;
use std::rc::Rc;

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
            // VIEW doesn't outlive the data
            let inner_view = unsafe { (view.builder)(&*(&*view.data as *const T)) };
            Lazy {
                data: view.data,
                builder: view.builder,
                inner: this.inner.update(inner_view),
            }
        }
    }
}
