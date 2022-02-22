use crate::kurbo::Size;
use crate::{app::AppHolder, AppDyn};

use druid_shell::{Application, WindowBuilder};

use crate::shell_handler::ShellHandler;

pub fn launch<A, Msg>(app: A)
where
    Msg: 'static,
    A: crate::Application<Msg = Msg> + 'static,
{
    let app = Box::new(app);
    _launch(app)
}

fn _launch(app: Box<dyn AppDyn>) {
    let application = Application::new().unwrap();
    let holder = AppHolder::new(app);

    let handler = ShellHandler::new(holder);
    let mut builder = WindowBuilder::new(application.clone());
    builder.set_title("Druidinho");
    builder.set_size(Size::new(400., 400.));
    builder.set_handler(Box::new(handler));
    let window = builder.build().unwrap();
    window.show();
    application.run(None);
}
