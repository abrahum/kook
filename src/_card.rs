// todo need rebuild

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Card {
    Card {
        theme: Option<String>,
        color: Option<String>,
        size: Option<String>,
        modules: Vec<CardModule>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CardModule {
    Header {
        text: CardItem,
    },
    Section {
        text: CardItem,
        accessory: CardItem,
        mode: String,
    },
    ImageGroup {
        elements: Vec<CardItem>,
    },
    Container {
        elements: Vec<CardItem>,
    },
    ActionGroup {
        elements: Vec<CardItem>,
    },
    Context {
        elements: Vec<CardItem>,
    },
    Divider,
    File {
        src: String,
        title: String,
        cover: String,
    },
    Audio {
        src: String,
        title: String,
        cover: String,
    },
    Video {
        src: String,
        title: String,
        cover: String,
    },
    Countdown {
        #[serde(rename = "endTime")]
        end_time: u64,
        #[serde(rename = "startTime")]
        start_time: u64,
        mode: String,
    },
    Invite {
        code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CardItem {
    PlainText {
        content: String,
        emoji: bool,
    },
    Kmarkdown {
        content: String,
    },
    Image {
        src: String,
        alt: String,
        size: String,
        circle: bool,
    },
    Bottom {
        theme: String,
        value: String,
        click: String,
        text: String,
    },
}
