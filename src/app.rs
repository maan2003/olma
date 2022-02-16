use std::any::Any;

use crate::{
    core::{AnyView, AnyWidget},
    widget_host::WidgetHost,
};

pub trait AppDyn {
    fn update_first(&mut self, msg: &dyn Any);
    fn update_second(&mut self, msg: Box<dyn Any>);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

pub trait Application {
    type Action;
    fn update(&mut self, msg: Self::Action);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

impl<Msg, A> AppDyn for A
where
    A: Application<Action = Msg>,
    Msg: Clone + 'static,
{
    fn update_first(&mut self, msg: &dyn Any) {
        match msg.downcast_ref::<Msg>() {
            Some(msg) => self.update(msg.clone()),
            None => eprintln!("Unknown Message: {:?}", msg),
        }
    }

    fn update_second(&mut self, msg: Box<dyn Any>) {
        match msg.downcast::<Msg>() {
            Ok(msg) => self.update(*msg),
            Err(msg) => eprintln!("Unknown Message: {:?}", msg),
        }
    }

    fn view<'a>(&'a self) -> AnyView<'a> {
        self.view()
    }
}

pub struct AppHolder {
    current: Box<dyn AppDyn>,
    next: Box<dyn AppDyn>,
    // borrows from current
    host: WidgetHost<'static>,
}

impl AppHolder {
    pub fn new<A>(left: A, right: A) -> Self
    where
        A: AppDyn + 'static,
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

    pub fn update(&mut self, msg: Box<dyn Any>) {
        take_mut::take(self, |mut this| {
            let mut current = this.current;
            let mut next = this.next;
            next.update_first(&msg);
            let next_view =
                unsafe { std::mem::transmute::<AnyView<'_>, AnyView<'static>>(next.view()) };
            this.host = this.host.update(next_view);
            current.update_second(msg);
            this.current = next;
            this.next = current;
            this
        });
    }
}
