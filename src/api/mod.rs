use crate::error::KookResult;
use crate::net::http::QueryBuilder;
use crate::{objects::*, structs::*, Kook};

macro_rules! http_api {
    ($fn_name: ident -> $rty: ty, $method: ident, $url: expr) => {
        pub async fn $fn_name(&self) -> KookResult<$rty> {
            let query = QueryBuilder::default();
            self.$method($url, query).await
        }
    };
    ($fn_name: ident -> $rty: ty, $method: ident, $url: expr, $($key: ident: $kty: ty),*) => {
        pub async fn $fn_name(&self, $($key: $kty),*) -> KookResult<$rty> {
            let mut query = QueryBuilder::default();
            $(
                query.push(stringify!($key), $key);
            )*
            self.$method($url, query).await
        }
    };
}

const GUILD: &str = "guild";

impl crate::Kook {
    http_api!(get_guild_list -> RespList<GuildShort>,
        get, vec![GUILD, "list"],
        page: Option<i32>,
        page_size: Option<i32>,
        sort: Option<&str>);
    http_api!(get_guild_view -> Guild,
        get, vec![GUILD, "view"],
        guild_id: &str);
    http_api!(get_guild_user_list -> GuildUserList,
        get, vec![GUILD, "user-list"],
        guild_id: &str,
        channel_id: Option<&str>,
        search: Option<&str>,
        role_id: Option<i32>,
        mobile_verified: bool,
        active_time: bool,
        joined_at: bool,
        page: Option<i32>,
        page_size: Option<i32>,
        filter_user_id: Option<&str>);
    http_api!(set_guild_user_nickname -> (),
        empty_post, vec![GUILD, "nickname"],
        guild_id: &str,
        user_id: Option<&str>,
        nickname: Option<&str>);
    http_api!(leave_guild -> (), empty_post, vec![GUILD, "leave"], guild: &str);
    http_api!(kickout_guild_user -> (), empty_post, vec![GUILD, "kickout"], guild: &str, target_id: &str);
}

const GUILD_MUTE: &str = "guild_mute";

impl Kook {
    pub async fn get_guild_mute_list(&self, guild_id: &str) -> KookResult<MuteList> {
        let mut query = QueryBuilder::default();
        query.push("guild_id", guild_id);
        query.push("return_type", "detail");
        self.get(vec![GUILD_MUTE, "list"], query).await
    }
    pub async fn create_guild_mute(&self, guild_id: &str, user_id: &str, ty: u8) -> KookResult<()> {
        let mut query = QueryBuilder::default();
        query.push("guild_id", guild_id);
        query.push("user_id", user_id);
        query.push("type", ty);
        self.empty_post(vec![GUILD_MUTE, "create"], query).await
    }
    pub async fn delete_guild_mute(&self, guild_id: &str, user_id: &str, ty: u8) -> KookResult<()> {
        let mut query = QueryBuilder::default();
        query.push("guild_id", guild_id);
        query.push("user_id", user_id);
        query.push("type", ty);
        self.empty_post(vec![GUILD_MUTE, "delete"], query).await
    }
}

const CHANNEL: &str = "channel";

impl Kook {
    pub async fn get_channel_list(
        &self,
        page: Option<i64>,
        page_size: Option<i64>,
        guild_id: &str,
        ty: Option<u8>,
    ) -> KookResult<RespList<ChannelShort>> {
        let mut query = QueryBuilder::default();
        query.push("page", page);
        query.push("page_size", page_size);
        query.push("guild_id", guild_id);
        query.push("type", ty);
        self.get(vec![CHANNEL, "list"], query).await
    }
    http_api!(get_channel_view -> ChannelView,
        get, vec![CHANNEL, "view"],
        target_id: &str);
    pub async fn create_channel(
        &self,
        guild_id: &str,
        parent_id: Option<&str>,
        name: &str,
        ty: Option<u8>,
        limit_amount: Option<i64>,
        voice_quality: Option<&str>,
        is_category: Option<i64>,
    ) -> KookResult<RespList<ChannelShort>> {
        let mut query = QueryBuilder::default();
        query.push("guild_id", guild_id);
        query.push("parent_id", parent_id);
        query.push("name", name);
        query.push("type", ty);
        query.push("limit_amount", limit_amount);
        query.push("voice_quality", voice_quality);
        query.push("is_category", is_category);
        self.post(vec![CHANNEL, "create"], query).await
    }
    http_api!(update_channel -> Channel,
        post, vec![CHANNEL, "update"],
        channel_id: &str,
        name: Option<&str>,
        topic: Option<&str>,
        slow_mode: Option<i64>);
    http_api!(delete_channel -> (),
        empty_post, vec![CHANNEL, "delete"],
        channel_id: &str);
    http_api!(get_channel_user_list -> Vec<User>,
        get, vec![CHANNEL, "user-list"],
        channel_id: &str);
    http_api!(move_channel_user -> (),
        empty_post, vec![CHANNEL, "move-user"],
        target_id: &str, user_id: &str);
}

const CHANNEL_ROLE: &str = "channel-role";

impl Kook {
    http_api!(get_channel_role -> ChannelRole,
        get, vec![CHANNEL_ROLE, "index"],
        channel_id: &str);
    // pub async fn create_channel_role ->
}

const MESSAGE: &str = "message";

impl Kook {
    pub async fn create_message(
        &self,
        ty: Option<u8>,
        target_id: &str,
        content: &str,
        quote: Option<&str>,
        nonce: Option<&str>,
        temp_target_id: Option<&str>,
    ) -> KookResult<MessageResp> {
        let mut query = QueryBuilder::default();
        query.push("target_id", target_id);
        query.push("content", content);
        query.push("type", ty);
        query.push("quote", quote);
        query.push("nonce", nonce);
        query.push("temp_target_id", temp_target_id);
        self.post(vec![MESSAGE, "create"], query).await
    }
    http_api!(update_messaeg -> (),
        empty_post, vec![MESSAGE, "update"],
        msg_id: &str,
        content: &str,
        quote: Option<&str>,
        temp_target_id: Option<&str>);
    http_api!(delete_message -> (),
        empty_post, vec![MESSAGE, "delete"],
        msg_id: &str);
}

const DIRECT_MESSAGE: &str = "direct-message";

impl crate::Kook {
    http_api!(get_direct_message_list -> RespList<DirectMessage>,
        get, vec![DIRECT_MESSAGE, "list"],
        chat_code: Option<&str>,
        target_id: Option<&str>,
        msg_id: Option<&str>,
        flag: Option<&str>,
        page: Option<u64>,
        page_size: Option<u64>);
    pub async fn create_direct_message(
        &self,
        target_id: Option<&str>,
        chat_code: Option<&str>,
        content: &str,
        ty: Option<u8>,
        quote: Option<&str>,
        nonce: Option<&str>,
    ) -> KookResult<MessageResp> {
        let mut query = QueryBuilder::default();
        query.push("target_id", target_id);
        query.push("chat_code", chat_code);
        query.push("content", content);
        query.push("type", ty);
        query.push("quote", quote);
        query.push("nonce", nonce);
        self.post(vec![DIRECT_MESSAGE, "create"], query).await
    }
    http_api!(update_direct_message-> (),
        empty_post, vec![DIRECT_MESSAGE, "update"],
        msg_id: Option<&str>,
        content: &str,
        quote: Option<&str>);
    http_api!(delete_direct_message -> (),
        empty_post, vec![DIRECT_MESSAGE, "delete"],
        msg_id: Option<&str>);
    // http_api!(get_direct_message_reaction_list) todo
    // http_api!(add_direct_message_reaction) todo
    // http_api!(delete_direct_message_reaction) todo
}

const USER: &str = "user";

impl crate::Kook {
    http_api!(get_me -> SelfUser, get, vec![USER, "me"]);
}

const GATEWAY: &str = "gateway";

impl crate::Kook {
    http_api!(get_gateway -> Gateway,
    get, vec![GATEWAY, "index"],
    compress: bool);
}

#[tokio::test]
async fn user_me() {
    let kook = crate::init();
    let me = kook.get_me().await;
    println!("{:?}", me);
}
