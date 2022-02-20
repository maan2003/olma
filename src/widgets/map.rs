use std::{
    any::{Any, TypeId},
    mem,
};

use druid_shell::{KeyEvent, TimerToken};

use crate::{core::*, widget::SingleChildContainer, EventCtx, MouseEvent, UiWidget};

pub struct Map<'a, T, U> {
    map: fn(T) -> U,
    passthru: bool,
    inner: AnyView<'a>,
}

struct MapWidget<T, U> {
    map: fn(T) -> U,
    passthru: bool,
    inner: AnyWidget,
}

impl<'a, T: 'static, U: 'static> CustomView<'a> for Map<'a, T, U> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Map<'static, T, U>>()
    }

    fn build(self) -> Box<dyn Widget> {
        Box::new(MapWidget {
            map: self.map,
            passthru: self.passthru,
            inner: self.inner.build(),
        })
    }
}

impl<U, T> CustomWidget for MapWidget<T, U>
where
    U: 'static,
    T: 'static,
{
    type View<'t> = Map<'t, T, U>;

    fn update<'a>(&mut self, view: Self::View<'a>) {
        self.passthru = view.passthru;
        self.map = view.map;
        self.inner.update(view.inner);
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        self
    }
}

impl<T: 'static, U: 'static> MapWidget<T, U> {
    fn check_msgs(&mut self, ctx: &mut EventCtx) {
        take_mut::take(ctx.messages, |mut msgs| {
            for msg in &mut msgs {
                take_mut::take(msg, |msg| match msg.downcast::<T>() {
                    Ok(msg) => Box::new((self.map)(*msg)),
                    Err(other_msg) => {
                        struct UnknownMsg;
                        if self.passthru {
                            other_msg
                        } else {
                            eprintln!("MapWidget: passthru is false, dropping message");
                            Box::new(UnknownMsg)
                        }
                    }
                })
            }
            msgs
        })
    }
}

impl<T, U> SingleChildContainer for MapWidget<T, U>
where
    U: 'static,
    T: 'static,
{
    type Child = AnyWidget;

    fn widget(&self) -> &Self::Child {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut Self::Child {
        &mut self.inner
    }

    fn init(&mut self, ctx: &mut EventCtx) {
        self.widget_mut().init(ctx)
    }

    fn mouse_down(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_down(ctx, event)
    }

    fn mouse_up(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_up(ctx, event)
    }

    fn mouse_move(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().mouse_move(ctx, event)
    }

    fn scroll(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.widget_mut().scroll(ctx, event)
    }

    fn key_down(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.widget_mut().key_down(ctx, event)
    }

    fn key_up(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.widget_mut().key_up(ctx, event)
    }

    fn timer(&mut self, ctx: &mut EventCtx, token: TimerToken) {
        self.widget_mut().timer(ctx, token)
    }
}
