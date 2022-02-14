use crate::core::*;

use std::any::TypeId;

struct ListView<'a, T> {
    list: &'a [T],
    child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>,
}

struct List<'a, T> {
    list: &'a [T],
    // I surrender to the compiler, the lifetimes got too complicated
    // I will just use heap allocation
    child: Box<dyn Fn(&T) -> AnyView<'a> + 'a>,
    children: Vec<AnyWidget<'a>>,
}

impl<'a, T> CustomView<'a> for ListView<'a, T>
where
    T: 'static,
{
    fn type_id(&self) -> TypeId {
        todo!()
    }

    fn build(self) -> Box<dyn Widget<'a>> {
        Box::new(WidgetWrap::<List<_>> {
            inner: List {
                list: self.list,
                child: self.child,
                children: Vec::new(),
            },
        })
    }
}

impl<'a, T> CustomWidget for List<'a, T>
where
    T: 'static,
{
    type View<'t> = ListView<'t, T>;
    type This<'t> = List<'t, T>;

    fn update<'orig, 'new>(
        mut this: Self::This<'orig>,
        view: Self::View<'new>,
    ) -> Self::This<'new> {
        this.children.truncate(view.list.len());
        let mut it = view.list.iter();
        let mut children = this
            .children
            .into_iter()
            .map(|w| {
                let view = (view.child)(it.next().unwrap());
                w.update(view)
            })
            .collect::<Vec<_>>();

        for elems in it {
            let view = (view.child)(elems);
            let widget = view.build();
            children.push(widget);
        }

        List {
            list: view.list,
            child: view.child,
            children,
        }
    }
}
