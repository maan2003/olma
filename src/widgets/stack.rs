use super::layout::stack as ui;
use crate::core::*;
use crate::widget_host::WidgetHost;

use std::any::TypeId;

pub struct Stack<'a> {
    children: Vec<AnyView<'a>>,
    axis: &'static dyn ui::Axis,
}

impl<'a> Stack<'a> {
    pub fn column() -> Stack<'a> {
        Stack {
            children: Vec::new(),
            axis: &ui::Vertical,
        }
    }

    pub fn row() -> Stack<'a> {
        Stack {
            children: Vec::new(),
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

impl<'a> CustomView<'a> for Stack<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Stack<'static>>()
    }

    fn build(self) -> Box<dyn Widget> {
        Box::new(StackWidget {
            ui: ui::Stack {
                children: self
                    .children
                    .into_iter()
                    .map(|c| WidgetHost::new(c.build()))
                    .collect(),
                axis: self.axis,
            },
        })
    }
}

struct StackWidget {
    ui: ui::Stack,
}

impl CustomWidget for StackWidget {
    type View<'t> = Stack<'t>;

    fn update<'a>(&mut self, view: Self::View<'a>) {
        self.ui.axis = view.axis;
        self.ui.children.truncate(view.children.len());
        let mut views = view.children.into_iter();
        for child in &mut self.ui.children {
            child.update(views.next().unwrap());
        }
        for child in views {
            self.ui.children.push(WidgetHost::new(child.build()));
        }
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
