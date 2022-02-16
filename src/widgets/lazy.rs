use crate::core::*;

use std::any::TypeId;
use std::rc::Rc;

struct LazyWidget<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
    // has references into data
    inner: AnyWidget<'static>,
}

struct Lazy<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
}

impl<'a, T: 'static> CustomView<'a> for Lazy<T> {
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget<'a>> {
        // VIEW doesn't outline the data
        let view = unsafe { (self.builder)(&*(&*self.data as *const T)) };
        Box::new(WidgetWrap::<LazyWidget<_>>::new(LazyWidget {
            inner: view.build(),
            data: self.data,
            builder: self.builder,
        }))
    }
}

impl<'a, T: 'static> CustomWidget for LazyWidget<T> {
    type View<'t> = Lazy<T>;
    type This<'t> = LazyWidget<T>;

    fn update<'orig, 'new>(this: Self::This<'orig>, view: Self::View<'new>) -> Self::This<'new> {
        if Rc::ptr_eq(&this.data, &view.data) && this.builder as usize == view.builder as usize {
            this
        } else {
            // VIEW doesn't outlive the data
            let inner_view = unsafe { (view.builder)(&*(&*view.data as *const T)) };
            LazyWidget {
                data: view.data,
                builder: view.builder,
                inner: this.inner.update(inner_view),
            }
        }
    }

    fn as_ui_widget<'x, 't>(this: &'t mut Self::This<'x>) -> &'t mut (dyn crate::UiWidget + 'x) {
        &mut this.inner
    }
}
