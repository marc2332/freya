// FlexWrap component - provides CSS flex-wrap like behavior for Freya
use freya_core::prelude::*;
use torin::{direction::Direction, prelude::Area, size::Size};

/// FlexWrap component that automatically wraps children to new rows
/// when they exceed the container width.
///
/// The component dynamically tracks its width using `on_sized` and automatically
/// adjusts the number of items per row when the window is resized.
///
/// # Example
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let cards: Vec<Element> = vec![
///         rect().width(Size::px(200.)).into_element(),
///         rect().width(Size::px(200.)).into_element(),
///         rect().width(Size::px(200.)).into_element(),
///     ];
///     
///     rsx! {
///         FlexWrap {
///             spacing: 16.0,
///             item_width: 200.0,
///             children: cards
///         }
///     }
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct FlexWrap {
    /// Spacing between items (horizontal and vertical gap)
    pub spacing: f32,
    /// Fixed width for each item
    pub item_width: f32,
    /// Child elements to wrap
    pub children: Vec<Element>,
    /// Diff key for React-like reconciliation
    key: DiffKey,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyExt for FlexWrap {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for FlexWrap {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

#[allow(non_snake_case)]
impl FlexWrap {
    /// Create a new FlexWrap instance
    pub fn new() -> Self {
        Self {
            spacing: 16.0,
            item_width: 200.0,
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    /// Set the spacing between items
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the width of each item
    pub fn item_width(mut self, width: f32) -> Self {
        self.item_width = width;
        self
    }

    /// Set the children elements
    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    /// Set the diff key for reconciliation
    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Render for FlexWrap {
    fn render(&self) -> impl IntoElement {
        // Track the container's actual width dynamically
        let mut container_size = use_state(Area::default);

        // Get actual width from the tracked area
        let container_width = container_size().width();

        // Use actual width if available, otherwise fallback to 1200px for initial render
        // The component will automatically re-render when on_sized fires with the real width
        let effective_width = if container_width > 0.0 {
            container_width
        } else {
            1200.0 // Fallback for initial render before on_sized fires
        };

        // Calculate how many items fit in one row:
        // Formula: (container_width + spacing) / (item_width + spacing)
        // The spacing is added to account for gaps between items
        let items_per_row = ((effective_width + self.spacing) / (self.item_width + self.spacing))
            .floor()
            .max(1.0) as usize; // Ensure at least 1 item per row

        // Group children into rows using chunks based on items_per_row
        // Each chunk becomes a horizontal row in the final layout
        let row_elements: Vec<Element> = self
            .children
            .chunks(items_per_row)
            .map(|row_items| {
                // For each row, wrap items in rects with fixed width
                let items: Vec<Element> = row_items
                    .iter()
                    .map(|item| {
                        // Wrap each item in a rect with the specified width
                        // This ensures consistent item sizing across the layout
                        rect()
                            .width(Size::px(self.item_width))
                            .child(item.clone())
                            .into_element()
                    })
                    .collect();

                // Create a horizontal row container
                rect()
                    .direction(Direction::Horizontal) // Items flow left to right
                    .spacing(self.spacing) // Gap between items
                    .width(Size::fill()) // Take full width
                    .children(items)
                    .into_element()
            })
            .collect();

        // Create the main vertical container that holds all rows
        // The on_sized callback tracks the container width and triggers re-renders
        rect()
            .on_sized(move |e: Event<SizedEventData>| {
                // Update the container size whenever layout changes
                container_size.set(e.area);
            })
            .direction(Direction::Vertical) // Rows stack top to bottom
            .spacing(self.spacing) // Gap between rows
            .width(Size::fill()) // Take full available width
            .children(row_elements) // Add all row containers
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
