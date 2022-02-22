use std::any::Any;

use druid_shell::{KeyEvent, TimerToken};

use crate::{core::*, widget::SingleChildContainer, EventCtx, MouseEvent, UiWidget};

pub struct Map<'a> {
    map: Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any>>,
    inner: AnyView<'a>,
}

pub struct MapWidget {
    map: Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any>>,
    inner: AnyWidget,
}

impl<'a> Map<'a> {
    pub fn new<T, U>(_map: fn(T) -> U, _inner: AnyView) -> Self {
        todo!()
    }
}

impl<'a> View<'a> for Map<'a> {
    type Widget = MapWidget;
    fn build(self) -> Self::Widget {
        MapWidget {
            map: self.map,
            inner: self.inner.build(),
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        widget.map = self.map;
        self.inner.update(&mut widget.inner);
    }
}

impl Widget for MapWidget {
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget {
        self
    }
}

impl MapWidget {
    fn check_msgs(&mut self, ctx: &mut EventCtx) {
        take_mut::take(ctx.messages, |mut msgs| {
            for msg in &mut msgs {
                take_mut::take(msg, |msg| (self.map)(msg));
            }
            msgs
        })
    }
}

impl SingleChildContainer for MapWidget {
    type Child = AnyWidget;

    fn widget(&self) -> &Self::Child {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut Self::Child {
        &mut self.inner
    }

    fn init(&mut self, ctx: &mut EventCtx) {
        self.widget_mut().init(ctx);
        self.check_msgs(ctx);
    }

    fn mouse_down(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_down(ctx, event);
        self.check_msgs(ctx);
    }

    fn mouse_up(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_up(ctx, event);
        self.check_msgs(ctx);
    }

    fn mouse_move(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_move(ctx, event);
        self.check_msgs(ctx);
    }

    fn scroll(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().scroll(ctx, event);
        self.check_msgs(ctx);
    }

    fn key_down(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.widget_mut().key_down(ctx, event);
        self.check_msgs(ctx);
    }

    fn key_up(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.widget_mut().key_up(ctx, event);
        self.check_msgs(ctx);
    }

    fn timer(&mut self, ctx: &mut EventCtx, token: TimerToken) {
        self.widget_mut().timer(ctx, token);
        self.check_msgs(ctx);
    }
}
