use crate::core::*;
use crate::ui_widgets::text as ui;

use std::any::TypeId;
use std::borrow::Cow;

pub struct Text<'a> {
    pub(crate) text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Text { text: text.into() }
    }
}

pub struct TextWidget {
    ui: ui::Text,
}

impl<'a> CustomView<'a> for Text<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Text<'static>>()
    }

    fn build(self) -> Box<dyn Widget> {
        Box::new(TextWidget {
            ui: ui::Text::new(self.text.as_ref()),
        })
    }
}

impl CustomWidget for TextWidget {
    type View<'t> = Text<'t>;

    fn update<'a>(&mut self, view: Self::View<'a>) {
        if self.ui.text() != view.text {
            self.ui.set_text(view.text.into_owned());
        }
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
