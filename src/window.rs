use std::any::Any;
use std::mem;

use crate::app::AppHolder;
use crate::core::AnyWidget;
use crate::kurbo::{Point, Size};
use crate::piet::Piet;

use crate::widget_host::{WidgetHost, WidgetState};
use crate::widgets::layout::LayoutState;
use crate::{BoxConstraints, EventCtx, LayoutCtx, PaintCtx, UiWidget};
use druid_shell::{IdleToken, KeyEvent, MouseEvent, Region, TimerToken, WindowHandle};

pub struct Window {
    handle: WindowHandle,
    root_state: WidgetState,
    layout_state: LayoutState,
    app: AppHolder,
    pub(crate) messages: Vec<Box<dyn Any>>,
}

impl Window {
    fn with_event_ctx<R>(&mut self, f: impl FnOnce(&mut WidgetHost, &mut EventCtx) -> R) -> R {
        let mut ctx = EventCtx {
            window: &self.handle,
            state: &mut self.root_state,
            layout_state: &self.layout_state,
            messages: &mut self.messages,
        };

        //if self.root_state.request_update {
        //self.root.update();
        //self.root_state.request_update = false;
        //}

        self.app.with_host(|w| f(w, &mut ctx))
    }

    pub fn new(handle: WindowHandle, app: AppHolder) -> Self {
        Window {
            handle,
            app,
            messages: Default::default(),
            layout_state: Default::default(),
            root_state: Default::default(),
        }
    }

    pub fn update(&mut self) {
        for msg in mem::take(&mut self.messages) {
            self.app.update(msg);
        }
    }

    pub fn window_connected(&mut self) {
        self.with_event_ctx(|chld, ctx| chld.init(ctx));
    }

    pub fn prepare_paint(&mut self) {
        let mut ctx = LayoutCtx {
            state: &self.root_state,
            layout_state: &self.layout_state,
            window: &self.handle,
        };
        let bc = BoxConstraints::tight(self.layout_state.size);
        self.app.with_host(|root| {
            root.layout(&mut ctx, bc);
            root.set_origin(Point::ZERO);
        })
    }

    pub fn paint(&mut self, piet: &mut Piet, _region: &Region) {
        let mut ctx = PaintCtx {
            state: &self.root_state,
            layout_state: &self.layout_state,
            render_ctx: piet,
        };

        self.app.with_host(|root| {
            root.paint(&mut ctx);
        })
    }

    pub fn size_changed(&mut self, new_size: Size) {
        self.layout_state.size = new_size;
        self.handle.invalidate();
    }

    pub fn mouse_down(&mut self, event: &MouseEvent) {
        let event = event.to_owned().into();
        self.with_event_ctx(|chld, ctx| chld.mouse_down(ctx, &event))
    }

    pub fn mouse_up(&mut self, event: &MouseEvent) {
        let event = event.to_owned().into();
        self.with_event_ctx(|chld, ctx| chld.mouse_up(ctx, &event))
    }

    pub fn mouse_move(&mut self, event: &MouseEvent) {
        //eprintln!("window mouse move {}", event.pos);
        let event = event.to_owned().into();
        self.with_event_ctx(|chld, ctx| chld.mouse_move(ctx, &event))
    }

    pub fn scroll(&mut self, event: &MouseEvent) {
        let event = event.to_owned().into();
        self.with_event_ctx(|chld, ctx| chld.scroll(ctx, &event))
    }

    pub fn key_down(&mut self, event: KeyEvent) -> bool {
        self.with_event_ctx(|chld, ctx| chld.key_down(ctx, &event));
        false
    }

    pub fn key_up(&mut self, event: KeyEvent) {
        self.with_event_ctx(|chld, ctx| chld.key_up(ctx, &event))
    }

    pub fn timer(&mut self, token: TimerToken) {
        self.with_event_ctx(|chld, ctx| chld.timer(ctx, token))
    }

    pub fn idle(&mut self, _token: IdleToken) {
        //self.with_event_ctx(|chld, ctx| chld.id(ctx, token))
    }
}
