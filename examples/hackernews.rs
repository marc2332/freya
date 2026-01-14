#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::{
    Duration,
    UNIX_EPOCH,
};

use freya::prelude::*;
use freya_query::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    pub by: String,
    pub time: i64,
    pub score: i32,
    pub descendants: Option<i32>,
    #[serde(rename = "type")]
    pub item_type: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetTopStoriesIds;

impl QueryCapability for GetTopStoriesIds {
    type Ok = Vec<i64>;
    type Err = Box<dyn std::error::Error + Send + Sync>;
    type Keys = ();

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let res = blocking::unblock(|| {
            let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
            let response = ureq::get(url).call().map_err(|e| Box::new(e) as Error)?;
            let data = response
                .into_body()
                .read_to_vec()
                .map_err(|e| Box::new(e) as Error)?;
            serde_json::from_slice::<Vec<i64>>(&data).map_err(|e| Box::new(e) as Error)
        })
        .await?;

        Ok(res.into_iter().take(30).collect())
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetStory;

impl QueryCapability for GetStory {
    type Ok = Story;
    type Err = Error;
    type Keys = i64;

    async fn run(&self, id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        let story = blocking::unblock(move || {
            let response = ureq::get(&url).call().map_err(|e| Box::new(e) as Error)?;
            let data = response
                .into_body()
                .read_to_vec()
                .map_err(|e| Box::new(e) as Error)?;
            serde_json::from_slice::<Story>(&data).map_err(|e| Box::new(e) as Error)
        })
        .await?;

        Ok(story)
    }
}

#[derive(PartialEq)]
struct StoryItem {
    id: i64,
}

impl StoryItem {
    fn new(id: i64) -> Self {
        Self { id }
    }
}

impl Component for StoryItem {
    fn render(&self) -> impl IntoElement {
        let story_query =
            use_query(Query::new(self.id, GetStory).stale_time(Duration::from_secs(600))); // Cache stories for 10 minutes

        match &*story_query.read().state() {
            QueryStateData::Pending => rect()
                .width(Size::fill())
                .height(Size::px(50.0))
                .center()
                .child("Loading story...")
                .into_element(),
            QueryStateData::Loading { .. } => rect()
                .width(Size::fill())
                .height(Size::px(50.0))
                .center()
                .child("Loading story...")
                .into_element(),
            QueryStateData::Settled { res: Ok(story), .. } => {
                let score_text = format!(
                    "{} points by {} {}",
                    story.score,
                    story.by,
                    format_time(story.time)
                );
                let comment_text = if let Some(descendants) = story.descendants {
                    format!("{} comments", descendants)
                } else {
                    "".to_string()
                };
                let url = story.url.clone();

                Button::new()
                    .width(Size::fill())
                    .child(
                        rect()
                            .width(Size::fill())
                            .padding((8.0, 16.0))
                            .child(story.title.clone())
                            .child(score_text)
                            .child(comment_text),
                    )
                    .maybe_some(url, |el, url| {
                        el.on_press(move |_| {
                            let _ = open::that(&url);
                        })
                    })
                    .into_element()
            }
            QueryStateData::Settled { res: Err(e), .. } => rect()
                .width(Size::fill())
                .height(Size::px(50.0))
                .center()
                .color((255, 0, 0))
                .child(format!("Error: {}", e))
                .into_element(),
        }
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.id)
    }
}

fn format_time(timestamp: i64) -> String {
    let dt = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(dt).unwrap_or(Duration::ZERO);

    let hours = duration.as_secs() / 3600;
    let days = hours / 24;

    if days > 0 {
        format!("{}d ago", days)
    } else if hours > 0 {
        format!("{}h ago", hours)
    } else {
        format!("{}m ago", duration.as_secs() / 60)
    }
}

fn app() -> impl IntoElement {
    let top_stories_query =
        use_query(Query::new((), GetTopStoriesIds).stale_time(Duration::from_secs(300)));

    let content: Element = match &*top_stories_query.read().state() {
        QueryStateData::Pending => rect()
            .width(Size::fill())
            .height(Size::px(50.0))
            .center()
            .child("Loading...")
            .into(),
        QueryStateData::Loading { .. } => rect()
            .width(Size::fill())
            .height(Size::px(50.0))
            .center()
            .child("Loading...")
            .into(),
        QueryStateData::Settled { res: Ok(ids), .. } => {
            let stories: Vec<Element> = ids.iter().map(|&id| StoryItem::new(id).into()).collect();

            ScrollView::new().children(stories).into()
        }
        QueryStateData::Settled { res: Err(e), .. } => rect()
            .width(Size::fill())
            .height(Size::px(50.0))
            .center()
            .color((255, 0, 0))
            .child(format!("Error: {}", e))
            .into(),
    };

    rect()
        .expanded()
        .background((240, 240, 240))
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(60.0))
                .background((255, 102, 0))
                .center()
                .child("Hacker News"),
        )
        .child(content)
}

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(800., 600.)
                .with_title("Hacker News"),
        ),
    );
}
