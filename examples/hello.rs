#![feature(type_alias_impl_trait, generic_associated_types)]
#![allow(clippy::needless_lifetimes)]

use olma::core::View;
use olma::piet::Color;
use olma::widgets::button::Button;
use olma::widgets::list::List;
use olma::widgets::stack::Stack;
use olma::widgets::text::Text;
use olma::{Application, ViewExt};

macro_rules! view_body_list {
    (
        (:view $view:ident)
        (:current $($curr:tt)*)
        , $($rest:tt)*
    ) => {{
        $view = $view.child(
            view! {
                $($curr)*
            }
        );
        view_body_list! {
            (:view $view)
            (:current )
            $($rest)*
        }
    }};
    (
        (:view $view:ident)
        (:current $($curr:tt)*)
        $tok:tt $($rest:tt)*
    ) => {
        view_body_list! {
            (:view $view)
            (:current $($curr)* $tok)
            $($rest)*
        }
    };
    (
        (:view $view:ident)
        (:current)
    ) => {};

    (
        (:view $view:ident)
        (:current $($curr:tt)*)
    ) => {
        $view = $view.child({
            view! {
                $($curr)*
            }
        });
    }
}

macro_rules! view_body_fields {
    (
        (:view $view:ident)
        // todo: not use expr here
        $f:ident : $v:expr,
        $($rest:tt)*
    ) => {{
        $view = $view.$f($v);
        view_body_fields! {
            (:view $view)
            $($rest)*
        }
    }};
    (
        (:view $view:ident)
        // todo: not use expr here
        $f:ident => $v:expr,
        $($rest:tt)*
    ) => {{
        $view = $view.$f(|| Box::new($v) as Box<dyn std::any::Any>);
        view_body_fields! {
            (:view $view)
            $($rest)*
        }
    }};
    (
        (:view $view:ident)
        // all done
    ) => {};
}

macro_rules! view_args {
    ((:func $f:path)
     (:parsed $($parsed:tt)*)
     (:current $($curr:tt)*)
     , $($rest:tt)*) => {
         view_args! {
             (:func $f)
             (:parsed $($parsed)* view! { $($curr)* },)
             (:current )
             $($rest)*
         }
     };
    ((:func $f:path)
     (:parsed $($parsed:tt)*)
     (:current $($curr:tt)*)
     $tok:tt $($rest:tt)*) => {
        view_args! {
            (:func $f)
            (:parsed $($parsed)* )
            (:current $($curr)* $tok)
            $($rest)*
        }
    };
    ((:func $f:path)
     (:parsed $($parsed:tt)*)
     (:current $($curr:tt)*)
     ) => {
         $f($($parsed)* view! { $($curr)* },)
    };

    ((:func $f:path)
     (:parsed $($parsed:tt)*)
     (:current )
     ) => {
         $f($($parsed)*)
    };
}

macro_rules! view_body {
    (
        (:view $view:ident)
        $name:ident =>
        $($rest:tt)*
    ) => {
        view_body_fields! {
            (:view $view)
            $name => $($rest)*
        }
    };
    (
        (:view $view:ident)
        $name:ident :
        $($rest:tt)*
    ) => {
        view_body_fields! {
            (:view $view)
            $name : $($rest)*
        }
    };

    (
        (:view $view:ident)
        $($rest:tt)*
    ) => {
        view_body_list! {
            (:view $view)
            (:current )
            $($rest)*
        }
    }
}

macro_rules! view {
    (for ($var:ident in $list:expr) {
        $($body:tt)*
    }) => {
        List::new($list, Box::new(|$var| {
            olma::core::AnyView::new(view! {
                $($body)*
            })
        }))
    };

    ($name:ident $(($($args:tt)*))?
        $({ $($rest:tt)* })?
        $(.$($calls:tt)*)?
    ) => {
        view! {
            $name::new $(($($args)*))?
            $({ $($rest)* })?
            $(.$($calls)*)?
        }
    };

    ($($name:ident)::+ $(($($args:tt)*))?
        $({ $($rest:tt)* })?
        $(.$($calls:tt)*)?
    ) => {{
        #[allow(unused_mut)]
        let mut view = view_args! {
            (:func $($name)::+)
            (:parsed )
            (:current )
            $($($args)*)?
        };
        $(view_body! {
            (:view view)
            $($rest)*
        })?
        view$(.$($calls)*)?
    }};

    (f $s:literal) => {
        format!($s)
    };

    ($($tt:tt)*) => {
        $($tt)*
    }
}

struct App {
    num: i32,
    list: Vec<i32>,
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
        view! {
            for (num in &self.list) {
                Text(f "Hello, {num}!")
            }
        }
    }
}

fn main() {
    olma::launch(App {
        num: 3,
        list: vec![1, 2, 3],
    });
}
