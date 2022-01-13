use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CardModule {
    Section { accessory: Box<CardModule> },
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
