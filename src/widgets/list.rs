use crate::{core::*, vbox_dyn, view_bump::VBox, widget_host::WidgetHost};

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

pub struct ListWidget {
    ui: Stack,
}

impl<'a> View<'a> for List<'a> {
    type Widget = ListWidget;

    fn build(mut self) -> Self::Widget {
        let items = (&mut *self.iter)
            .map(|w| WidgetHost::new(w.build()))
            .collect();

        ListWidget {
            ui: Stack {
                children: items,
                axis: &Vertical,
            },
        }
    }

    fn update(mut self, widget: &mut Self::Widget) {
        let mut idx = 0;
        for view in &mut *self.iter {
            if idx < widget.ui.children.len() {
                widget.ui.children[idx].update(view);
            } else {
                widget.ui.children.push(WidgetHost::new(view.build()));
            }
            idx += 1;
        }
        widget.ui.children.truncate(idx);
    }
}

impl Widget for ListWidget {
    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
