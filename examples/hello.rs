use olma::core::AnyView;
use olma::widgets::button::Button;
use olma::widgets::text::Text;
use olma::Application;

struct App {
    num: i32,
}

impl Application for App {
    type Action = ();

    fn update(&mut self, msg: Self::Action) {
        self.num += 1;
    }

    fn view<'a>(&'a self) -> AnyView<'a> {
        AnyView::new(Button::new(format!("current:{}", self.num)).on_click(|| Box::new(())))
    }
}

fn main() {
    olma::launch(App { num: 10 }, App { num: 10 }).unwrap();
}
