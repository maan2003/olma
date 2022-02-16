use crate::core::*;
use crate::ui_widgets::button as ui;

use std::any::{Any, TypeId};
use std::borrow::Cow;

use super::text::Text;

pub struct Button<'a> {
    text: Text<'a>,
    on_click: Option<fn() -> Box<dyn Any>>,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: Text::new(text),
            on_click: None,
        }
    }

    pub fn click(mut self, f: fn() -> Box<dyn Any>) -> Self {
        self.on_click = Some(f);
        self
    }
}

pub struct ButtonWidget<'a> {
    ui: ui::Button<'a>,
}

impl<'a> CustomView<'a> for Button<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<ButtonWidget<'static>>()
    }

    fn build(self) -> Box<dyn Widget<'a>> {
        Box::new(WidgetWrap::<ButtonWidget>::new(ButtonWidget {
            ui: ui::Button::new(self.text.text).on_click(self.on_click),
        }))
    }
}

impl<'a> CustomWidget for ButtonWidget<'a> {
    type View<'t> = Button<'t>;

    type This<'t> = ButtonWidget<'t>;

    fn update<'orig, 'new>(this: Self::This<'orig>, view: Self::View<'new>) -> Self::This<'new> {
        ButtonWidget {
            ui: ui::Button {
                text: this.ui.text.update(AnyView::new(view.text)),
                on_click: view.on_click,
                hovered: this.ui.hovered,
            },
        }
    }

    fn as_ui_widget<'b, 't>(this: &'t mut Self::This<'b>) -> &'t mut (dyn crate::UiWidget + 'b) {
        &mut this.ui
    }
}
