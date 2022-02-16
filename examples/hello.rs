#![allow(clippy::needless_lifetimes)]

use std::fmt::Arguments;

use olma::core::{AnyView, View};
use olma::widgets::button::Button;
use olma::widgets::text::{self, Text};
use olma::{AppDyn, Application};

struct App {
    num: i32,
}

macro_rules! view_body {
    (($v:ident)  on $event:ident : $f:expr, $($rest:tt)*) => {{
        $v = $v.$event(|| Box::new($f) as Box<dyn std::any::Any>);
        view_body! {
            ($v) $($rest)*
        }
    }};

    (($v:ident) $feild:ident : $val:expr, $($rest:tt)*) => {{
        $v.$feild(view! {$val});
        view_body! {
            ($v) $($rest)*
        }
    }};
    (($v:ident)) => {}
}

macro_rules! view {
    (
        $ty:ident ($($arg:expr),* $(,)?) $({
            $($body:tt)*
        })?
        $(where $($bind:ident = $val:expr;)*)?
    ) => {
        AnyView::new({
            $($(let $bind = $val;)*)?
            #[allow(unused)]
            let mut view = $ty::new($(view!{$arg}),*);
            view_body! {
                (view)
                $($($body)*)?
            }
            view
        })
    };
    ($fmt:literal) => {
        format!($fmt)
    };
    ($other:expr) => { $other }
}

#[derive(Clone, Copy)]
enum Msg {
    Increment,
}

impl Application for App {
    type Action = Msg;

    fn update(&mut self, msg: Self::Action) {
        match msg {
            Msg::Increment => self.num += 1,
        }
    }

    fn view<'a>(&'a self) -> impl View<'a> {
        Text::new(format!("Hello world"))
    }
}

fn main() {
    olma::launch(App { num: 10 }, App { num: 10 }).unwrap();
}
