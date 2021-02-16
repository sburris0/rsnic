use serde::Deserialize;
use std::fmt;

// TODO: ContentItem trait, impl Display for Vec<T: ContentItem>
/// Video type that API responses are parsed into
/// The Invidious API is documented [here](https://github.com/iv-org/documentation/blob/master/API.md).
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub author: String,
    pub description: String,
    pub published: i64,
    pub length_seconds: i32,
}

impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = self.length_seconds / 60;
        let seconds = self.length_seconds % 60;
        write!(
            f,
            "\"{}\" ({}:{:0>2}) - {}",
            self.title, minutes, seconds, self.author
        )
    }
}
