use freya_core::prelude::*;
use torin::prelude::{
    Area,
    CursorPoint,
    Size2D,
    Vector2D,
};

/// Starts native drag-and-drop from its children.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     NativeDrag::new(|| TransferData::text("Hello from Freya!"))
///         .preview(|| "Dragging text".into_element())
///         .child("Drag me")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct NativeDrag {
    children: Vec<Element>,
    data: NoArgCallback<TransferData>,
    actions: Vec<DropAction>,
    preview: Option<NoArgCallback<Element>>,
    preview_size: Option<Size2D>,
    on_finished: Option<EventHandler<Result<NativeDragResult, DragError>>>,
    enabled: bool,
    drag_threshold: f64,
    layout: LayoutData,
    key: DiffKey,
}

impl NativeDrag {
    /// Creates a drag source with a data factory called when dragging starts.
    pub fn new(data: impl Into<NoArgCallback<TransferData>>) -> Self {
        Self {
            children: Vec::new(),
            data: data.into(),
            actions: vec![DropAction::Copy],
            preview: None,
            preview_size: None,
            on_finished: None,
            enabled: true,
            drag_threshold: 5.,
            layout: LayoutData::default(),
            key: DiffKey::default(),
        }
    }

    /// Allowed drop actions. See [DropAction].
    pub fn actions(mut self, actions: impl IntoIterator<Item = DropAction>) -> Self {
        self.actions = actions.into_iter().collect();
        self
    }

    /// Custom preview, rendered once. Defaults to a snapshot of the source.
    pub fn preview(mut self, preview: impl Into<NoArgCallback<Element>>) -> Self {
        self.preview = Some(preview.into());
        self
    }

    /// Preview size in logical pixels. Defaults to the source size.
    pub fn preview_size(mut self, width: f32, height: f32) -> Self {
        self.preview_size = Some(Size2D::new(width, height));
        self
    }

    /// Called when dragging completes, is cancelled, or fails.
    pub fn on_finished(
        mut self,
        on_finished: impl Into<EventHandler<Result<NativeDragResult, DragError>>>,
    ) -> Self {
        self.on_finished = Some(on_finished.into());
        self
    }

    /// Enables native dragging. Defaults to true.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Drag threshold in logical pixels. Defaults to 5.
    pub fn drag_threshold(mut self, distance: f64) -> Self {
        self.drag_threshold = distance.max(0.);
        self
    }
}

impl ChildrenExt for NativeDrag {
    fn children_mut(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for NativeDrag {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for NativeDrag {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for NativeDrag {}

impl ComponentOwned for NativeDrag {
    fn render(self) -> impl IntoElement {
        let mut press = use_state(|| None::<(CursorPoint, Vector2D)>);
        let mut dragging = use_state(|| false);
        let mut area = use_state(Area::default);
        let threshold = self.drag_threshold;
        let data = self.data;
        let preview = self.preview;
        let preview_size = self.preview_size;
        let on_finished = self.on_finished;
        let actions = self.actions;

        if !self.enabled {
            press.set_if_modified(None);
        }

        rect()
            .layout(self.layout)
            .children(self.children)
            .on_sized(move |event: Event<SizedEventData>| area.set_if_modified(event.area))
            .maybe(self.enabled, |el| {
                el.on_pointer_down(move |event: Event<PointerEventData>| {
                    if event.is_primary() && !*dragging.peek() {
                        press.set_if_modified(Some((
                            event.global_location(),
                            event.element_location().to_vector().to_f32(),
                        )));
                    }
                })
                .on_global_pointer_up(move |_| press.set_if_modified(None))
                .on_global_pointer_move(move |event: Event<PointerEventData>| {
                    if *dragging.peek() {
                        return;
                    }
                    let Some((origin, offset)) = *press.peek() else {
                        return;
                    };

                    let distance = event.global_location() - origin;
                    if distance.length() < threshold {
                        return;
                    }
                    press.set_if_modified(None);

                    let area = area();
                    let preview = match &preview {
                        Some(preview) => NativeDragPreview::Element {
                            element: preview.call(),
                            size: preview_size.unwrap_or(area.size),
                            offset,
                        },
                        None => NativeDragPreview::Snapshot { area, offset },
                    };

                    let window = Platform::get().current_window();
                    let data = data.call();
                    let actions = actions.clone();
                    let on_finished = on_finished.clone();
                    dragging.set_if_modified(true);
                    event.prevent_default();

                    spawn(async move {
                        let result = window.start_drag(data, actions, Some(preview)).await;
                        dragging.set_if_modified(false);

                        if let Some(on_finished) = on_finished {
                            on_finished.call(result);
                        }
                    });
                })
            })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
