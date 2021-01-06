use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmoticonsResponse {
    #[serde(rename = "_links")]
    links: Option<Links>,
    #[serde(rename = "_pages")]
    pages: i64,
    #[serde(rename = "_total")]
    total: i64,
    pub emoticons: Vec<Emoticon>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Emoticon {
    pub id: i64,
    pub name: String,
    pub height: i64,
    pub width: i64,
    pub public: bool,
    pub hidden: bool,
    pub modifier: bool,
    pub offset: Option<String>,
    pub margins: Option<String>,
    pub css: Option<String>,
    pub owner: Owner,
    pub urls: EmoteUrls,
    pub status: i64,
    pub usage_count: i64,
    pub created_at: String,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EmoteUrls {
    #[serde(rename = "1")]
    one: String,
    #[serde(rename = "2")]
    two: Option<String>,
    #[serde(rename = "4")]
    four: Option<String>,
}

impl fmt::Display for EmoteUrls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = if let Some(u) = &self.four {
            u
        } else if let Some(u) = &self.two {
            u
        } else {
            &self.one
        };

        write!(f, "https:{}", url)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Owner {
    #[serde(rename = "_id")]
    id: i64,
    name: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    next: String,
    #[serde(rename = "self")]
    links_self: String,
    prev: String,
}
