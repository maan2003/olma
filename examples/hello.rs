#![feature(type_alias_impl_trait, generic_associated_types)]
#![allow(clippy::needless_lifetimes)]

use olma::core::View;
use olma::piet::Color;
use olma::widgets::lazy::LazyData;
use olma::widgets::*;
use olma::{Application, ViewExt};

struct App {
    num: i32,
    list: LazyData<Vec<i32>>,
}

enum Msg {
    Add,
    Remove,
}

impl Application for App {
    type Msg = Msg;
    type View<'a> = impl View<'a>;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Add => {
                self.num += 1;
                self.list.push(self.num);
            }
            Msg::Remove => {
                self.list.pop();
            }
        }
    }

    fn view<'a>(&'a self) -> Self::View<'a> {
        Lazy::new(&self.list, || {
            Stack::column()
                .child(List::new(
                    self.list.iter().map(|i| Text::new(format!("{}", i))),
                ))
                .child(
                    Stack::row()
                        .child(Button::new("Add").click(|| Msg::Add))
                        .child(Button::new("Remove").click(|| Msg::Remove)),
                )
                .background(Color::WHITE)
        })
    }
}

fn main() {
    olma::launch(App {
        num: 3,
        list: LazyData::new(vec![1, 2, 3]),
    });
}
