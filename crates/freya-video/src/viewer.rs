use freya_components::loader::CircularLoader;
use freya_core::{
    elements::image::{
        ImageData,
        ImageHolder,
        image,
    },
    integration::*,
    prelude::*,
};

use crate::{
    PlaybackState,
    VideoPlayer,
};

/// Renders the current frame of a [`VideoPlayer`].
#[derive(PartialEq)]
pub struct VideoViewer {
    player: VideoPlayer,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,

    key: DiffKey,
}

impl VideoViewer {
    pub fn new(player: VideoPlayer) -> Self {
        Self {
            player,
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for VideoViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for VideoViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for VideoViewer {}

impl ImageExt for VideoViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for VideoViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl Component for VideoViewer {
    fn render(&self) -> impl IntoElement {
        match (self.player.frame(), self.player.state()) {
            (Some(frame), _) => image(ImageHolder::new(frame.image))
                .accessibility(self.accessibility.clone())
                .a11y_role(AccessibilityRole::Video)
                .a11y_focusable(true)
                .layout(self.layout.clone())
                .image_data(self.image_data.clone())
                .into_element(),
            (None, PlaybackState::Errored) => "Failed to decode video".into_element(),
            (None, _) => rect()
                .layout(self.layout.clone())
                .center()
                .child(CircularLoader::new())
                .into(),
        }
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
