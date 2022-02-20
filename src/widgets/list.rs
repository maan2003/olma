use crate::{core::*, widget_host::WidgetHost};

use std::{any::TypeId, marker::PhantomData};

use super::layout::stack::{Stack, Vertical};

pub struct List<'a, T> {
    list: &'a [T],
    child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>,
}

impl<'a, T> List<'a, T> {
    pub fn new(list: &'a [T], child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>) -> Self {
        Self { list, child }
    }
}

struct ListWidget<T> {
    ui: Stack,
    _type: PhantomData<T>,
}

impl<'a, T> CustomView<'a> for List<'a, T>
where
    T: 'static,
{
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget> {
        let items = self
            .list
            .iter()
            .map(|item| WidgetHost::new((self.child)(item).build()))
            .collect();

        Box::new(ListWidget::<T> {
            ui: Stack {
                children: items,
                axis: &Vertical,
            },
            _type: PhantomData,
        })
    }
}

impl<T> CustomWidget for ListWidget<T>
where
    T: 'static,
{
    type View<'t> = List<'t, T>;

    fn update<'a>(&mut self, view: Self::View<'a>) {
        self.ui.children.truncate(view.list.len());
        let mut it = view.list.iter();
        for child in &mut self.ui.children {
            let view = (view.child)(it.next().unwrap());
            child.update(view);
        }
        for elems in it {
            let view = (view.child)(elems);
            let widget = view.build();
            self.ui.children.push(WidgetHost::new(widget));
        }
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
