mod client;
mod player;
mod viewer;

pub use self::{
    client::{
        VideoClient,
        VideoEvent,
        VideoFrame,
        VideoSource,
    },
    player::{
        PlaybackState,
        VideoPlayer,
        use_video,
    },
    viewer::{
        Video,
        VideoElement,
        VideoViewer,
        video,
    },
};
