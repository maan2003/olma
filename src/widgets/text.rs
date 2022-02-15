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

    fn build(self) -> Box<dyn Widget<'a>> {
        Box::new(WidgetWrap::<TextWidget>::new(TextWidget {
            ui: ui::Text::new(self.text.as_ref()),
        }))
    }
}

impl CustomWidget for TextWidget {
    type View<'t> = Text<'t>;

    type This<'t> = TextWidget;

    fn update<'orig, 'new>(
        mut this: Self::This<'orig>,
        view: Self::View<'new>,
    ) -> Self::This<'new> {
        if this.ui.text() != view.text {
            this.ui.set_text(view.text.into_owned());
        }
        this
    }

    fn as_ui_widget<'a, 't>(this: &'t mut Self::This<'a>) -> &'t mut (dyn crate::UiWidget + 'a) {
        &mut this.ui
    }
}
