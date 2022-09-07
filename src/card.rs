use serde::{Deserialize, Serialize};

use crate::prelude::KookError;

pub fn cards_decode(s: &str) -> Result<Cards, KookError> {
    serde_json::from_str(s).map_err(|e| KookError::SerdeJsonError(e))
}

pub fn cards_encode(cards: &Cards) -> String {
    serde_json::to_string(cards).unwrap()
}

pub type Cards = Vec<Card>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[default]
    Primary,
    Success,
    Danger,
    Warning,
    Info,
    Secondary,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Size {
    Xs,
    Sm,
    Md,
    #[default]
    Lg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub theme: Option<String>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub modules: Vec<CardModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CardModule {
    Header {
        text: CardItem,
    },
    Section {
        // plain-text|kmarkdown|paragraph
        text: CardItem,
        // image|button
        accessory: Option<CardItem>,
        // left|right
        mode: Option<String>,
    },
    ImageGroup {
        // image
        elements: Vec<CardItem>,
    },
    Container {
        // image
        elements: Vec<CardItem>,
    },
    ActionGroup {
        // botton
        elements: Vec<CardItem>,
    },
    Context {
        // plain-text|kmarkdown|image
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
        // day,hour,second
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
        size: Size,
        circle: bool,
    },
    Bottom {
        theme: Theme,
        value: String,
        click: String,
        text: String,
    },
    Paragraph {
        col: u8,
        // only PlainText or Kmarkdown
        fields: Vec<CardItem>,
    },
}
