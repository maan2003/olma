use druid_shell::piet::Color;

use crate::{core::*, widgets::background::Background};

pub trait ViewExt<'a>: View<'a> + Sized {
    fn background(self, color: Color) -> Background<'a> {
        Background::new(self).background(color)
    }

    fn any(self) -> AnyView<'a> {
        AnyView::new(self)
    }
}

impl<'a, V> ViewExt<'a> for V where V: View<'a> {}
