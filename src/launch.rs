use crate::app::{self, AppHolder};
use crate::kurbo::Size;

use druid_shell::{Application, Error as PlatformError, WindowBuilder};

use crate::shell_handler::ShellHandler;

pub fn launch<Msg, A>(app: A, app2: A) -> Result<(), PlatformError>
where
    Msg: 'static + Clone,
    A: app::Application<Action = Msg> + 'static,
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
