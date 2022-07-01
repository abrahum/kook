use std::vec;

use crate::error::KHLResult;
use crate::net::http::QueryBuilder;
use crate::structs::*;

macro_rules! http_api {
    ($fn_name: ident -> $rty: ty, $method: ident,$url: expr, $($key: ident: $kty: ty),*) => {
        pub async fn $fn_name(&self, $($key: $kty),*) -> KHLResult<$rty> {
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
}

const DIRECT_MESSAGE: &str = "direct-message";

impl crate::Kook {
    pub async fn send_direct_message(
        &self,
        target_id: Option<&str>,
        chat_code: Option<&str>,
        content: &str,
        ty: Option<i32>,
        quote: Option<&str>,
        nonce: Option<&str>,
    ) -> KHLResult<MessageResp> {
        let mut query = QueryBuilder::default();
        query.push("target_id", target_id);
        query.push("chat_code", chat_code);
        query.push("content", content);
        query.push("type", ty);
        query.push("quote", quote);
        query.push("nonce", nonce);
        self.post(vec![DIRECT_MESSAGE, "create"], query).await
    }
}

const GATEWAY: &str = "gateway";

impl crate::Kook {
    http_api!(get_gateway -> Gateway,
    get, vec![GATEWAY, "index"],
    compress: bool);
}
