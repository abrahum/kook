use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Event<T> {
    pub channel_type: String,
    #[serde(rename = "type")]
    pub ty: u8,
    pub target_id: String,
    pub author_id: String,
    pub msg_id: String,
    pub content: String,
    pub msg_timestamp: i64,
    pub nonce: String,
    pub extra: T,
}

impl Event<EventExtra> {
    pub fn down_case<T>(self) -> Option<Event<T>>
    where
        T: TryFrom<EventExtra>,
    {
        match self.extra.try_into() {
            Ok(extra) => Some(Event {
                channel_type: self.channel_type,
                ty: self.ty,
                target_id: self.target_id,
                author_id: self.author_id,
                msg_id: self.msg_id,
                content: self.content,
                msg_timestamp: self.msg_timestamp,
                nonce: self.nonce,
                extra,
            }),
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EventExtra {
    System(SystemExtra),
    Message(MessageExtra),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "body", rename_all = "snake_case")]
pub enum SystemExtra {
    AddedReaction {
        channel_id: String,
        emoji: Emoji,
        user_id: String,
        msg_id: String,
    },
    DeletedReaction {
        channel_id: String,
        emoji: Emoji,
        user_id: String,
        msg_id: String,
    },
    UpdateMessage {
        channel_id: String,
        content: String,
        mention: Vec<String>,
        mention_all: bool,
        mention_here: bool,
        mention_roles: Vec<i32>,
        updated_at: i64,
        msg_id: String,
    },
    DeletedMessage {
        channel_id: String,
        msg_id: String,
    },
    AddedChannel(Channel),
    UpdatedChannel(Channel),
    DeletedChannel {
        id: String,
        deleted_at: i64,
    },
    PinnedMessage {
        channel_id: String,
        operator_id: String,
        msg_id: String,
    },
    UnpinnedMessage {
        channel_id: String,
        operator_id: String,
        msg_id: String,
    },
    UpdatedPrivateMessage {
        author_id: String,
        target_id: String,
        msg_id: String,
        content: String,
        updated_at: i64,
        chat_code: String,
    },
}

impl TryFrom<EventExtra> for SystemExtra {
    type Error = ();

    fn try_from(value: EventExtra) -> Result<Self, Self::Error> {
        match value {
            EventExtra::System(value) => Ok(value),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageExtra {
    #[serde(rename = "type")]
    pub ty: i32,
    pub guild_id: Option<String>,
    pub channel_name: Option<String>,
    pub mention: Vec<String>,
    pub mention_all: bool,
    pub mention_roles: Vec<i32>,
    pub mention_here: bool,
    pub author: User,
}

impl TryFrom<EventExtra> for MessageExtra {
    type Error = ();

    fn try_from(value: EventExtra) -> Result<Self, Self::Error> {
        match value {
            EventExtra::Message(value) => Ok(value),
            _ => Err(()),
        }
    }
}

#[test]
fn extra_de_test() {
    let s = r#"{
        "type": "updated_private_message",
        "body": {
          "author_id": "2862900000",
          "target_id": "2862900000",
          "msg_id": "93262503-xxxx-0d814f7b416a",
          "content": "asdaaad",
          "updated_at": 1612778254183,
          "chat_code": "xxxxxxxxxxxxxxxxx"
        }
      }"#;
    let e: EventExtra = serde_json::from_str(s).unwrap();
    println!("{:#?}", e);
}
