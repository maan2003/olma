use crate::{core::*, vbox_dyn, view_bump::VBox, UiWidget};

/// Does not propogate updates down the tree.
pub struct Const<'a> {
    builder: VBox<'a, dyn Fn() -> AnyView<'a>>,
}

pub fn Const<'a, V>(builder: impl Fn() -> V + 'static) -> Const<'a>
where
    V: View<'a>,
{
    Const {
        builder: vbox_dyn!(move || AnyView::new(builder()), _),
    }
}

pub struct ConstWidget {
    inner: AnyWidget,
}

impl<'a> View<'a> for Const<'a> {
    type Widget = ConstWidget;
    fn build(self) -> Self::Widget {
        ConstWidget {
            inner: (self.builder)().build(),
        }
    }

    fn update(self, _widget: &mut Self::Widget) {
        // no-op
    }
}

impl Widget for ConstWidget {
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget {
        &mut self.inner
    }
}
