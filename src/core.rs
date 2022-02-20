use std::any::{type_name, TypeId};
use std::mem::{self, ManuallyDrop};
use std::ptr;

use druid_shell::kurbo::Size;
use druid_shell::{KeyEvent, TimerToken};

use crate::{BoxConstraints, EventCtx, LayoutCtx, MouseEvent, PaintCtx, UiWidget};

// SAFETY: build must only be called once and must be treated as if it is `self`
// caller must not drop `self`
pub unsafe trait View<'a>: 'a {
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
    fn build(&mut self) -> Box<dyn Widget>;
}

// VIEW describes a widget, widget can be built from a view
pub trait CustomView<'a>: 'a {
    // TODO(safety): this trait should be unsafe=
    fn type_id(&self) -> TypeId;
    fn build(self) -> Box<dyn Widget>;
}

pub trait CustomWidget: 'static {
    type View<'t>: CustomView<'t>;
    fn update<'a>(&mut self, view: Self::View<'a>);
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget;
}

/// # Safety
///  - it should be safe to transmute the widget from 'a to 'b after call to `update`
pub unsafe trait Widget: 'static {
    fn view_type_id(&self) -> TypeId;
    /// update the references in widget from 'a to 'b in the widget
    ///
    /// # Safety
    ///  - caller needs to use ManuallyDrop to ensure view is not dropped by the caller
    ///  - *mut dyn View<'b> must be treated as if it was a `dyn View<'b>`
    unsafe fn update<'b>(&mut self, view: *mut dyn View<'b>);
    fn as_ui_widget(&mut self) -> &mut (dyn UiWidget);
}

unsafe impl<'a, C: CustomView<'a>> View<'a> for C {
    fn type_id(&self) -> TypeId {
        <C as CustomView>::type_id(self)
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }

    fn build(&mut self) -> Box<dyn Widget> {
        let this = self as *mut Self;
        // SAFETY: caller must ensure that the view is not dropped by the caller
        let this = unsafe { ptr::read(this) };
        this.build()
    }
}

unsafe impl<W> Widget for W
where
    W: CustomWidget,
{
    fn view_type_id(&self) -> TypeId {
        TypeId::of::<W::View<'static>>()
    }

    unsafe fn update<'a>(&mut self, view: *mut dyn View<'a>) {
        // SAFETY: view is valid pointer
        // FIXME: use
        if (*view).type_name() != type_name::<W::View<'static>>() {
            panic!(
                "view type mismatch: expected {:?}, got {:?}",
                type_name::<W::View<'static>>(),
                (*view).type_name()
            );
        }
        let view_ptr = view.cast::<W::View<'a>>();
        let view = ptr::read(view_ptr);
        // make segfaults very likely if caller tries to use view after the call to update
        ptr::write_bytes(view_ptr, 0xaa, 1);
        <W as CustomWidget>::update(self, view);
    }

    fn as_ui_widget(&mut self) -> &mut (dyn UiWidget) {
        W::as_ui_widget(self)
    }
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

    pub fn update<'b>(&mut self, mut view: AnyView<'b>) {
        unsafe {
            // FIXME
            self.inner
                .update(mem::transmute(&mut **view.inner as *mut dyn View<'b>));
        }
        VBox::forget(view.inner);
    }

    pub fn view_type_id(&self) -> TypeId {
        self.inner.view_type_id()
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

use crate::view_bump::{self, VBox};

pub struct AnyView<'a> {
    inner: VBox<'a, dyn View<'a>>,
}

impl<'a> AnyView<'a> {
    pub fn new<W: View<'a>>(inner: W) -> Self {
        let bump = view_bump::current();
        let value = bump.alloc(inner) as &mut dyn View<'a> as *mut dyn View<'a>;

        AnyView {
            inner: unsafe { VBox::from_raw(bump, value) },
        }
    }

    pub fn build(mut self) -> AnyWidget {
        let widget = self.inner.build();
        // not drop the View
        VBox::forget(self.inner);
        AnyWidget { inner: widget }
    }

    pub fn type_id(&self) -> TypeId {
        self.inner.type_id()
    }

    pub fn type_name(&self) -> &'static str {
        self.inner.type_name()
    }
}
