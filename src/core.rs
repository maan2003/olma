use std::any::{Any, TypeId};

use druid_shell::kurbo::Size;
use druid_shell::{KeyEvent, TimerToken};

use crate::{vbox_dyn, BoxConstraints, EventCtx, LayoutCtx, MouseEvent, PaintCtx, UiWidget};

pub trait View<'a>: 'a {
    type Widget: Widget;
    fn build(self) -> Self::Widget;
    // widget is garaunteed to have same type_id as `widget_type_id`
    fn update(self, widget: &mut Self::Widget);
}

trait DynView<'a>: 'a {
    fn build(&mut self) -> Box<dyn Widget>;
    fn update(&mut self, widget: &mut dyn Widget);
}

struct DynViewWrap<V>(Option<V>);

impl<'a, V: View<'a>> DynView<'a> for DynViewWrap<V> {
    fn build(&mut self) -> Box<dyn Widget> {
        Box::new(self.0.take().unwrap().build())
    }

    fn update(&mut self, widget: &mut dyn Widget) {
        self.0
            .take()
            .unwrap()
            .update(widget.as_any().downcast_mut::<V::Widget>().unwrap());
    }
}

pub trait AsAny {
    fn as_any(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Widget: AsAny + 'static {
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget;
}

pub struct AnyWidget {
    inner: Box<dyn Widget>,
}

impl AnyWidget {
    pub fn new<W: Widget>(inner: W) -> Self {
        AnyWidget {
            inner: Box::new(inner),
        }
    }

    pub fn as_ui_widget(&mut self) -> &mut dyn UiWidget {
        self.inner.as_ui_widget()
    }

    pub fn update(&mut self, view: AnyView) {
        view.update(self);
    }
}

use crate::view_bump::VBox;

pub struct AnyView<'a> {
    inner: VBox<'a, dyn DynView<'a>>,
    widget_type_id: TypeId,
}

impl<'a> AnyView<'a> {
    pub fn new<V: View<'a>>(inner: V) -> Self {
        AnyView {
            inner: vbox_dyn!(DynViewWrap(Some(inner)), dyn DynView<'a>),
            widget_type_id: TypeId::of::<V::Widget>(),
        }
    }

    pub fn build(mut self) -> AnyWidget {
        let widget = self.inner.build();
        AnyWidget { inner: widget }
    }

    pub fn update(mut self, widget: &mut AnyWidget) {
        self.inner.update(&mut *widget.inner);
    }

    pub fn widget_type_id(&self) -> TypeId {
        self.widget_type_id
    }
}

impl UiWidget for AnyWidget {
    fn init(&mut self, ctx: &mut EventCtx) {
        self.inner.as_ui_widget().init(ctx);
    }

    fn mouse_down(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.inner.as_ui_widget().mouse_down(ctx, event);
    }

    fn mouse_up(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.inner.as_ui_widget().mouse_up(ctx, event);
    }

    fn mouse_move(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.inner.as_ui_widget().mouse_move(ctx, event);
    }

    fn scroll(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.inner.as_ui_widget().scroll(ctx, event);
    }

    fn key_down(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.inner.as_ui_widget().key_down(ctx, event);
    }

    fn key_up(&mut self, ctx: &mut EventCtx, event: &KeyEvent) {
        self.inner.as_ui_widget().key_up(ctx, event);
    }

    fn timer(&mut self, ctx: &mut EventCtx, token: TimerToken) {
        self.inner.as_ui_widget().timer(ctx, token);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: BoxConstraints) -> Size {
        self.inner.as_ui_widget().layout(ctx, bc)
    }

    fn paint(&mut self, ctx: &mut PaintCtx) {
        self.inner.as_ui_widget().paint(ctx);
    }
}
