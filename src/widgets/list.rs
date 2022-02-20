use crate::{core::*, vbox_dyn, view_bump::VBox, widget_host::WidgetHost};

use std::any::TypeId;

use super::layout::stack::{Stack, Vertical};

pub struct List<'a> {
    iter: VBox<'a, dyn Iterator<Item = AnyView<'a>> + 'a>,
}

impl<'a> List<'a> {
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator + 'a,
        I::Item: View<'a>,
    {
        let children = children.into_iter().map(AnyView::new);
        List {
            iter: vbox_dyn!(children, dyn Iterator<Item = AnyView<'a>> + 'a),
        }
    }
}

struct ListWidget {
    ui: Stack,
}

impl<'a> CustomView<'a> for List<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<List<'static>>()
    }

    fn build(mut self) -> Box<dyn Widget> {
        let items = (&mut *self.iter)
            .map(|w| WidgetHost::new(w.build()))
            .collect();

        Box::new(ListWidget {
            ui: Stack {
                children: items,
                axis: &Vertical,
            },
        })
    }
}

impl CustomWidget for ListWidget {
    type View<'t> = List<'t>;

    fn update<'a>(&mut self, mut view: Self::View<'a>) {
        let mut cnt = 0;
        for (view, child) in (&mut *view.iter).zip(self.ui.children.iter_mut()) {
            child.update(view);
            cnt += 1;
        }

        self.ui.children.truncate(cnt);
        for child in &mut *view.iter {
            self.ui.children.push(WidgetHost::new(child.build()));
        }
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
