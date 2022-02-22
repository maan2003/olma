#![allow(clippy::needless_lifetimes)]

use olma::core::AnyView;
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

    fn view<'a>(&'a self) -> AnyView<'a> {
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
        .any()
    }
}

fn main() {
    olma::launch(App {
        num: 3,
        list: LazyData::new(vec![1, 2, 3]),
    });
}
