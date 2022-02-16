#![feature(generic_associated_types)]
#![allow(clippy::needless_lifetimes, dead_code)]

mod box_constraints;
mod contexts;
mod launch;
mod mouse;
mod shell_handler;
mod widget;
// mod widget_ext;
mod app;
pub mod core;
mod rc2;
mod run;
mod widget_host;
pub mod widgets;
mod ui_widgets;
mod window;

pub use app::{Application, AppDyn};
pub use box_constraints::BoxConstraints;
pub use contexts::{EventCtx, LayoutCtx, PaintCtx};
pub use launch::launch;
pub use mouse::MouseEvent;
pub use widget::UiWidget;
pub use window::Window;

use app::AppHolder;
pub use druid_shell::{self as shell, kurbo, piet};
