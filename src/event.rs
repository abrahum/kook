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

    pub fn author(&self) -> Option<&User> {
        match &self.extra {
            EventExtra::GroupMessage(g) => Some(&g.author),
            EventExtra::PersonMessage(p) => Some(&p.author),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EventExtra {
    System(SystemExtra),
    GroupMessage(GroupMessageExtra),
    PersonMessage(PersonMessageExtra),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "body", rename_all = "snake_case")]
pub enum SystemExtra {
    // Channel
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
    UpdatedMessage {
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
    // private
    UpdatedPrivateMessage {
        msg_id: String,
        author_id: String,
        target_id: String,
        content: String,
        chat_code: String,
        updated_at: i64,
    },
    DeletedPrivateMessage {
        msg_id: String,
        author_id: String,
        target_id: String,
        chat_code: String,
        deleted_at: i64,
    },
    PrivateAddedReaction {
        msg_id: String,
        user_id: String,
        chat_code: String,
        emoji: Emoji,
    },
    PrivateDeletedReaction {
        msg_id: String,
        user_id: String,
        chat_code: String,
        emoji: Emoji,
    },
    // guild member
    JoinedGuild {
        user_id: String,
        joined_at: i64,
    },
    ExitedGuild {
        user_id: String,
        exited_at: i64,
    },
    UpdateGuildMember {
        user_id: String,
        nickname: String,
    },
    GuildMemberOnline {
        user_id: String,
        event_time: i64,
        guilds: Vec<String>,
    },
    GuildMemberOffline {
        user_id: String,
        event_time: i64,
        guilds: Vec<String>,
    },
    // role
    AddedRole(Role),
    DeletedRole(Role),
    UpdatedRole(Role),
    // guild
    UpdateGuild {
        // bad
        id: String,
        name: String,
        user_id: String,
        icon: String,
        notify_type: i64,
        region: String,
        enable_open: i64,
        open_id: i64,
        default_channel_id: String,
        welcome_channel_id: String,
    },
    DeletedGuild {
        id: String,
        name: String,
        user_id: String,
        icon: String,
        notify_type: i64,
        region: String,
        enable_open: i64,
        open_id: i64,
        default_channel_id: String,
        welcome_channel_id: String,
    },
    AddedBlockList {
        operator_id: String,
        remark: String,
        user_id: String,
    },
    DeletedBlockList {
        operator_id: String,
        user_id: String,
    },
    // user
    JoinedChannel {
        user_id: String,
        channel_id: String,
        joined_at: i64,
    },
    ExitedChannel {
        user_id: String,
        channel_id: String,
        exited_at: String,
    },
    UserUpdated {
        user_id: String,
        username: String,
        avatar: String,
    },
    SelfJoinedGuild {
        guild_id: String,
    },
    SelfExitedGuild {
        guild_id: String,
    },
    MessageBtnClick {
        msg_id: String,
        user_id: String,
        value: String,
        target_id: String,
        user_info: User,
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
pub struct GroupMessageExtra {
    #[serde(rename = "type")]
    pub ty: i32,
    pub guild_id: String,
    pub channel_name: String,
    pub mention: Vec<String>,
    pub mention_all: bool,
    pub mention_roles: Vec<i32>,
    pub mention_here: bool,
    pub author: User,
}

impl TryFrom<EventExtra> for GroupMessageExtra {
    type Error = ();

    fn try_from(value: EventExtra) -> Result<Self, Self::Error> {
        match value {
            EventExtra::GroupMessage(value) => Ok(value),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonMessageExtra {
    #[serde(rename = "type")]
    pub ty: i32,
    pub author: User,
    pub mention: Vec<String>,
    pub mention_all: bool,
    pub mention_roles: Vec<i32>,
    pub mention_here: bool,
}

impl TryFrom<EventExtra> for PersonMessageExtra {
    type Error = ();

    fn try_from(value: EventExtra) -> Result<Self, Self::Error> {
        match value {
            EventExtra::PersonMessage(value) => Ok(value),
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
