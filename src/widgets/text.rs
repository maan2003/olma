use crate::ui_widgets::text as ui;
use crate::{core::*, UiWidget};

use std::borrow::Cow;

pub struct Text<'a> {
    pub(crate) text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Text { text: text.into() }
    }
}

pub fn Text<'a>(text: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text::new(text)
}

pub struct TextWidget {
    ui: ui::Text,
}

impl<'a> View<'a> for Text<'a> {
    type Widget = TextWidget;

    fn build(self) -> Self::Widget {
        TextWidget {
            ui: ui::Text::new(self.text.as_ref()),
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        if widget.ui.text() != self.text {
            widget.ui.set_text(self.text.into_owned());
        }
    }
}

impl Widget for TextWidget {
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget {
        &mut self.ui
    }
}
