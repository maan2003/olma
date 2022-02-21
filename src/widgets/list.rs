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

pub fn List<'a, I>(children: I) -> List<'a>
where
    I: IntoIterator + 'a,
    I::Item: View<'a>,
{
    List::new(children)
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
        let mut idx = 0;
        for view in &mut *view.iter {
            if idx < self.ui.children.len() {
                self.ui.children[idx].update(view);
            } else {
                self.ui.children.push(WidgetHost::new(view.build()));
            }
            idx += 1;
        }
        self.ui.children.truncate(idx);
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
