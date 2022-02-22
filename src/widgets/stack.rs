use super::layout::stack as ui;
use crate::core::*;
use crate::view_bump::VVec;
use crate::widget_host::WidgetHost;

pub fn Column<'a>() -> Stack<'a> {
    Stack::column()
}

pub fn Row<'a>() -> Stack<'a> {
    Stack::row()
}

pub struct Stack<'a> {
    children: VVec<'a, AnyView<'a>>,
    axis: &'static dyn ui::Axis,
}

impl<'a> Stack<'a> {
    pub fn column() -> Stack<'a> {
        Stack {
            children: VVec::new(),
            axis: &ui::Vertical,
        }
    }

    pub fn row() -> Stack<'a> {
        Stack {
            children: VVec::new(),
            axis: &ui::Horizontal,
        }
    }

    pub fn child<V>(mut self, widget: V) -> Self
    where
        V: View<'a>,
    {
        self.children.push(AnyView::new(widget));
        self
    }
}

impl<'a> View<'a> for Stack<'a> {
    type Widget = StackWidget;
    fn build(self) -> Self::Widget {
        StackWidget {
            ui: ui::Stack {
                children: self
                    .children
                    .into_iter()
                    .map(|c| WidgetHost::new(c.build()))
                    .collect(),
                axis: self.axis,
            },
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        widget.ui.axis = self.axis;
        widget.ui.children.truncate(self.children.len());
        let mut views = self.children.into_iter();
        for child in &mut widget.ui.children {
            child.update(views.next().unwrap());
        }
        for child in views {
            widget.ui.children.push(WidgetHost::new(child.build()));
        }
    }
}

pub struct StackWidget {
    ui: ui::Stack,
}

impl Widget for StackWidget {
    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
