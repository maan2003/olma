use crate::view_bump::VBox;
use crate::{core::*, vbox_dyn, UiWidget};

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct LazyData<T> {
    version: u64,
    id: u64,
    data: T,
}

impl<T> LazyData<T> {
    pub fn new(data: T) -> LazyData<T> {
        LazyData {
            version: 0,
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            data,
        }
    }

    pub fn tick(&mut self) {
        self.version += 1;
    }
}

impl<T> std::ops::Deref for LazyData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for LazyData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.version += 1;
        &mut self.data
    }
}

pub struct LazyWidget {
    version: u64,
    id: u64,
    inner: AnyWidget,
}

pub struct Lazy<'a> {
    id: u64,
    version: u64,
    builder: VBox<'a, dyn Fn() -> AnyView<'a> + 'a>,
}

pub fn Lazy<'a, T, V>(value: &LazyData<T>, builder: impl Fn() -> V + 'a) -> Lazy<'a>
where
    V: View<'a>,
{
    Lazy::new(value, builder)
}

impl<'a> Lazy<'a> {
    pub fn new<T, V>(value: &LazyData<T>, builder: impl Fn() -> V + 'a) -> Self
    where
        V: View<'a>,
    {
        let id = value.id;
        let version = value.version;
        let builder = vbox_dyn!(
            move || AnyView::new(builder()),
            dyn Fn() -> AnyView<'a> + 'a
        );
        Lazy {
            id,
            version,
            builder,
        }
    }
}

impl<'a> View<'a> for Lazy<'a> {
    type Widget = LazyWidget;
    fn build(self) -> LazyWidget {
        LazyWidget {
            id: self.id,
            version: self.version,
            inner: (self.builder)().build(),
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        if widget.id != self.id || widget.version != self.version {
            widget.inner = (self.builder)().build();
            widget.id = self.id;
            widget.version = self.version;
            widget.inner.update((self.builder)());
        }
    }
}

impl Widget for LazyWidget {
    fn as_ui_widget(&mut self) -> &mut dyn UiWidget {
        self.inner.as_ui_widget()
    }
}
