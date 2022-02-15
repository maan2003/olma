use crate::{
    core::{AnyView, AnyWidget},
    widget_host::WidgetHost,
};

pub trait Application {
    type Action;
    fn update(&mut self, msg: Self::Action);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

pub struct AppHolder<T> {
    current: Box<dyn Application<Action = T>>,
    next: Box<dyn Application<Action = T>>,
    // borrows from current
    host: WidgetHost<'static>,
}

impl<T: Clone> AppHolder<T> {
    pub fn new<A>(left: A, right: A) -> Self
    where
        A: Application<Action = T> + 'static,
    {
        let current = Box::new(left);
        let next = Box::new(right);
        let widget = unsafe {
            std::mem::transmute::<AnyWidget<'_>, AnyWidget<'static>>(current.view().build())
        };
        let host = WidgetHost::new(widget);

        Self {
            current,
            next,
            host,
        }
    }

    pub fn with_host<R>(&mut self, f: impl FnOnce(&mut WidgetHost) -> R) -> R {
        f(&mut self.host)
    }

    pub fn update(&mut self, msg: T) {
        take_mut::take(self, |mut this| {
            let mut current = this.current;
            let mut next = this.next;
            next.update(msg.clone());
            let next_view =
                unsafe { std::mem::transmute::<AnyView<'_>, AnyView<'static>>(next.view()) };
            this.host = this.host.update(next_view);
            current.update(msg);
            this.current = next;
            this.next = current;
            this
        });
    }
}
