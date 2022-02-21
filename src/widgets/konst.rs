use std::any::TypeId;

use crate::{core::*, vbox_dyn, view_bump::VBox};

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

struct ConstWidget {
    inner: AnyWidget,
}

impl<'a> CustomView<'a> for Const<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Const<'static>>()
    }

    fn build(self) -> Box<dyn Widget> {
        Box::new(ConstWidget {
            inner: (self.builder)().build(),
        })
    }
}

impl CustomWidget for ConstWidget {
    type View<'t> = Const<'t>;

    fn update<'a>(&mut self, _view: Self::View<'a>) {
        // no-op
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.inner
    }
}
