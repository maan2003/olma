use std::any::Any;

use crate::{
    view_bump,
    core::{AnyView, View},
    widget_host::WidgetHost,
};

pub trait AppDyn {
    fn update(&mut self, msg: Box<dyn Any>);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

pub trait Application: 'static {
    type Msg;
    type View<'a>: View<'a>;
    fn update(&mut self, msg: Self::Msg);
    fn view<'a>(&'a self) -> Self::View<'a>;
}

impl<Msg, A> AppDyn for A
where
    A: Application<Msg = Msg>,
    Msg: 'static,
{
    fn update(&mut self, msg: Box<dyn Any>) {
        match msg.downcast::<Msg>() {
            Ok(msg) => self.update(*msg),
            Err(msg) => eprintln!("Unknown Message: {:?}", msg),
        }
    }

    fn view<'a>(&'a self) -> AnyView<'a> {
        AnyView::new(self.view())
    }
}

pub struct AppHolder {
    app: Box<dyn AppDyn>,
    host: WidgetHost,
}

impl AppHolder {
    pub fn new(app: Box<dyn AppDyn>) -> Self {
        view_bump::new();
        let widget = app.view().build();
        view_bump::reset();
        let host = WidgetHost::new(widget);

        Self { app, host }
    }

    pub fn with_host<R>(&mut self, f: impl FnOnce(&mut WidgetHost) -> R) -> R {
        f(&mut self.host)
    }

    pub fn update(&mut self, msg: Box<dyn Any>) {
        self.app.update(msg);
        view_bump::new();
        let next_view = self.app.view();
        self.host.update(next_view);
        view_bump::reset();
    }
}
