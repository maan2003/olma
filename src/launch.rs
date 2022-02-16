use crate::app::{self, AppHolder};
use crate::core::View;
use crate::kurbo::Size;

use druid_shell::{Application, Error as PlatformError, WindowBuilder};

use crate::shell_handler::ShellHandler;

pub fn launch<D, Msg, U, V, VV>(d1: D, d2: D, update: U, view: V) -> Result<(), PlatformError>
where
    Msg: 'static + Clone,
    D: 'static,
    U: 'static + Fn(&mut D, Msg),
    V: 'static + for<'a> Fn(&'a D) -> V<'a>,
    V: for<'a> View<'a>
{
    let application = Application::new()?;
    let holder = AppHolder::new(app, app2);

    let handler = ShellHandler::new(holder);
    let mut builder = WindowBuilder::new(application.clone());
    builder.set_title("Druidinho");
    builder.set_size(Size::new(400., 400.));
    builder.set_handler(Box::new(handler));
    let window = builder.build()?;
    window.show();
    application.run(None);
    Ok(())
}
