use crate::{
    net::http::QueryBuilder,
    prelude::{ChannelShort, GuildUserList, KookResult, RespList},
};

use super::{CHANNEL, GUILD};

impl crate::Kook {
    /// ### Example:
    /// ```rust
    /// kook.guild_user_list_getter("guild_id")
    ///     .channel_id("channel_id")
    ///     .get()
    ///     .await
    /// ```
    pub fn guild_user_list_getter<'a>(&'a self, guild_id: &str) -> GuildUserListGetter<'a> {
        let mut getter = GuildUserListGetter(QueryBuilder::default(), self);
        getter.0.push("guild_id", guild_id);
        getter
    }
    pub fn create_channel_poster<'a>(
        &'a self,
        guild_id: &str,
        name: &str,
    ) -> CreateChannelPoster<'a> {
        let mut poster = CreateChannelPoster(QueryBuilder::default(), self);
        poster.0.push("guild_id", guild_id);
        poster.0.push("name", name);
        poster
    }
}

macro_rules! set_fn {
    ($fname: ident, $attr: ident: $aty: ty) => {
        pub fn $fname(mut self, $attr: $aty) -> Self {
            self.0.push(stringify!($attr), $attr);
            self
        }
    };
}

pub struct GuildUserListGetter<'a>(QueryBuilder, &'a crate::Kook);

impl<'a> GuildUserListGetter<'a> {
    pub fn channel_id(mut self, channel_id: &str) -> Self {
        self.0.push("channel_id", channel_id);
        self
    }
    set_fn!(role_id, role_id: i32);
    set_fn!(search, search: &str);
    set_fn!(mobile_verified, mobile_verified: bool);
    set_fn!(active_time, active_time: bool);
    set_fn!(joined_at, joined_at: bool);
    set_fn!(page, set_page: i32);
    set_fn!(page_size, page_size: i32);
    set_fn!(filter_user_id, filter_user_id: &str);
    pub async fn get(self) -> KookResult<GuildUserList> {
        self.1.get([GUILD, "user-list"], self.0).await
    }
}

pub struct CreateChannelPoster<'a>(QueryBuilder, &'a crate::Kook);

impl<'a> CreateChannelPoster<'a> {
    pub fn ty(mut self, ty: u8) -> Self {
        self.0.push("type", ty);
        self
    }
    set_fn!(parent_id, parent_id: &str);
    set_fn!(limit_amount, limit_amount: i64);
    set_fn!(voice_quality, voice_quality: &str);
    set_fn!(is_category, is_category: i64);
    pub async fn post(self) -> KookResult<RespList<ChannelShort>> {
        self.1.post([CHANNEL, "create"], self.0).await
    }
}
