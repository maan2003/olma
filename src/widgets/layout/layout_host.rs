use crate::core::{AnyView, AnyWidget};
use crate::kurbo::{Affine, Point, Rect, Size};
use crate::piet::RenderContext;
use crate::widget::SingleChildContainer;
use crate::{BoxConstraints, EventCtx, LayoutCtx, MouseEvent, PaintCtx, UiWidget};

/// Manages the position of a child widget.
pub struct LayoutHost {
    state: LayoutState,
    debug_needs_set_origin: bool,
    pub(crate) child: AnyWidget,
}

/// State related to the layout of a particular widget.
#[derive(Clone, Debug, Default)]
pub(crate) struct LayoutState {
    pub(crate) size: Size,
    pub(crate) origin: Point,
    pub(crate) hovered: bool,
}

impl LayoutHost {
    /// Create a new `LayoutHost` for the given child.
    ///
    /// After this widget is laid out, the parent *must* call
    /// [`LayoutHost::set_origin`] in order to set the child's position.
    pub fn new(child: AnyWidget) -> Self {
        LayoutHost {
            child,
            state: LayoutState::default(),
            debug_needs_set_origin: true,
        }
    }

    pub fn update<'b>(&mut self, view: AnyView<'b>) {
        view.update(&mut self.child);
    }

    /// Set the position of the child, relative to the origin of the parent.
    pub fn set_origin(&mut self, point: Point) {
        self.state.origin = point;
        self.debug_needs_set_origin = false;
    }

    /// The child's size.
    pub fn size(&self) -> Size {
        self.state.size
    }

    fn contains(&self, mouse: &MouseEvent) -> bool {
        Rect::from_origin_size(self.state.origin, self.state.size).contains(mouse.pos)
    }

    fn propagate_mouse_if_needed(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        f: impl FnOnce(&mut AnyWidget, &mut EventCtx, &MouseEvent),
    ) {
        let was_hovered = self.state.hovered;
        self.state.hovered = self.contains(event);
        let mut mouse = event.clone();
        mouse.pos -= self.state.origin.to_vec2();
        let mut child_ctx = EventCtx {
            state: ctx.state,
            layout_state: &mut self.state,
            window: ctx.window,
            messages: ctx.messages,
        };
        if was_hovered || ctx.layout_state.hovered || child_ctx.state.has_mouse_focus() {
            f(&mut self.child, &mut child_ctx, &mouse);
        }
    }
}

impl SingleChildContainer for LayoutHost {
    type Child = AnyWidget;

    fn widget(&self) -> &Self::Child {
        &self.child
    }

    fn widget_mut(&mut self) -> &mut Self::Child {
        &mut self.child
    }
    fn mouse_down(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.propagate_mouse_if_needed(ctx, event, |child, ctx, e| child.mouse_down(ctx, e));
    }

    fn mouse_move(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.propagate_mouse_if_needed(ctx, event, |child, ctx, e| child.mouse_move(ctx, e));
    }

    fn mouse_up(&mut self, ctx: &mut EventCtx, event: &MouseEvent) {
        self.propagate_mouse_if_needed(ctx, event, |child, ctx, e| child.mouse_up(ctx, e));
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: BoxConstraints) -> Size {
        self.debug_needs_set_origin = true;
        let mut child_ctx = LayoutCtx {
            layout_state: &mut self.state,
            state: ctx.state,
            window: ctx.window,
        };
        self.state.size = self.child.layout(&mut child_ctx, bc);
        //TODO: validate that size matches constraints?
        self.state.size
    }

    fn paint(&mut self, ctx: &mut PaintCtx) {
        if self.debug_needs_set_origin {
            panic!("Missing call to set_origin");
        }
        let mut child_ctx = PaintCtx {
            render_ctx: ctx.render_ctx,
            state: ctx.state,
            layout_state: &self.state,
        };
        child_ctx.with_save(|ctx| {
            let layout_origin = ctx.layout_state.origin.to_vec2();
            ctx.transform(Affine::translate(layout_origin));
            self.child.paint(ctx);
        });
    }
}
