#[derive(Clone)]
struct Model;
#[derive(Copy, Clone)]
struct Msg;
struct Widget<'a>(&'a ());
struct View<'a>(&'a ());

fn build_widget<'a>(view: View<'a>) -> Widget<'a> {
    Widget(view.0)
}
// update the model referenced by the widget
fn update_widget<'a, 'b>(_widget: Widget<'a>, view: View<'b>) -> Widget<'b> {
    Widget(view.0)
}
fn update_data(_data: &mut Model, _msg: Msg) {}

fn view<'a>(_model: &'a Model) -> View<'a> {
    View(&())
}

fn main() {
    let mut current = Model;
    let mut next = Model;

    let current_view = view(&current);
    let mut widget = build_widget(current_view);
    loop {
        // assume current and next are the same
        let msg = Msg;
        update_data(&mut next, msg);
        let widget2 = update_widget(widget, view(&next));
        update_data(&mut current, msg);

        let msg = Msg;
        update_data(&mut current, msg);
        widget = update_widget(widget2, view(&current));
        update_data(&mut next, msg);
        // invariant: current and next will stay the same
    }
}
