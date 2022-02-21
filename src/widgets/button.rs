use crate::core::*;
use crate::ui_widgets::button as ui;

use std::any::{Any, TypeId};
use std::borrow::Cow;

use super::text::Text;

pub struct Button<'a> {
    text: Text<'a>,
    // most likely a zero-sized type
    on_click: Option<Box<dyn Fn() -> Box<dyn Any>>>,
}

pub fn Button<'a>(text: impl Into<Cow<'a, str>>) -> Button<'a> {
    Button {
        text: Text(text),
        on_click: None,
    }
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: Text::new(text),
            on_click: None,
        }
    }

    pub fn click<M>(mut self, f: impl Fn() -> M + 'static) -> Self
    where
        M: 'static,
    {
        self.on_click = Some(Box::new(move || Box::new(f())));
        self
    }
}

pub struct ButtonWidget {
    ui: ui::Button,
}

impl<'a> CustomView<'a> for Button<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<ButtonWidget>()
    }

    fn build(self) -> Box<dyn Widget> {
        Box::new(ButtonWidget {
            ui: ui::Button::new(self.text.text).on_click(self.on_click),
        })
    }
}

impl<'a> CustomWidget for ButtonWidget {
    type View<'t> = Button<'t>;

    fn update<'view>(&mut self, view: Self::View<'view>) {
        self.ui.text.update(AnyView::new(view.text));
        self.ui.on_click = view.on_click;
    }

    fn as_ui_widget(&mut self) -> &mut dyn crate::UiWidget {
        &mut self.ui
    }
}
