use crate::core::*;

use std::any::TypeId;
use std::rc::Rc;

struct LazyWidget<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
    // has references into data
    inner: AnyWidget,
}

struct Lazy<T> {
    data: Rc<T>,
    builder: for<'a> fn(&'a T) -> AnyView<'a>,
}

impl<'a, T: 'static> CustomView<'a> for Lazy<T> {
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget> {
        // VIEW doesn't outline the data
        let view = unsafe { (self.builder)(&*(&*self.data as *const T)) };
        Box::new(LazyWidget {
            inner: view.build(),
            data: self.data,
            builder: self.builder,
        })
    }
}

impl<'a, T: 'static> CustomWidget for LazyWidget<T> {
    type View<'t> = Lazy<T>;

    fn update<'new>(&mut self, view: Self::View<'new>) {
        if !Rc::ptr_eq(&self.data, &view.data) || self.builder as usize != view.builder as usize {
            // VIEW doesn't outlive the data
            let inner_view = unsafe { (view.builder)(&*(&*view.data as *const T)) };
            self.data = view.data;
            self.builder = view.builder;
            self.inner.update(inner_view);
        }
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.inner
    }
}
