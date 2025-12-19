use freya_core::prelude::*;
use torin::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TileStatus {
    #[default]
    Idle,
    Hovering,
}

#[derive(Clone, PartialEq)]
pub struct Tile {
    children: Vec<Element>,
    leading: Option<Element>,
    on_select: Option<EventHandler<()>>,
    key: DiffKey,
}

impl KeyExt for Tile {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Tile {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Tile {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            leading: None,
            on_select: None,
            key: DiffKey::None,
        }
    }

    pub fn leading(mut self, leading: impl Into<Element>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    pub fn on_select(mut self, on_select: impl FnMut(()) + 'static) -> Self {
        self.on_select = Some(EventHandler::new(on_select));
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Render for Tile {
    fn render(&self) -> impl IntoElement {
        let mut status = use_state(|| TileStatus::Idle);

        let on_press = {
            let on_select = self.on_select.clone();
            move |e: Event<PressEventData>| {
                if let Some(on_select) = &on_select {
                    e.stop_propagation();
                    on_select.call(());
                }
            }
        };

        let on_pointer_enter = {
            move |_| {
                *status.write() = TileStatus::Hovering;
            }
        };

        let on_pointer_leave = {
            move |_| {
                *status.write() = TileStatus::Idle;
            }
        };

        rect()
            .direction(Direction::Horizontal)
            .padding(8.)
            .spacing(8.)
            .cross_align(Alignment::center())
            .on_press(on_press)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .maybe_child(
                self.leading
                    .clone()
                    .map(|leading| rect().padding(Gaps::new_all(4.0)).child(leading)),
            )
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
