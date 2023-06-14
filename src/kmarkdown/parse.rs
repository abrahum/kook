use super::KMDItem;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Item {
    pub start: usize,
    pub end: usize,
    pub body: ItemBody,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum ItemBody {
    #[default]
    Text,
    SoftBreak,  // \n \r \r\n
    HardBreak,  // \n\n \r\r \r\n\r\n
    Blod,       // **
    Italic,     // *
    Deleted,    // ~~
    LinkStart,  // [
    LinkMiddle, // ](
    LinkEnd,    // )
    Divider,    // ---
    Ref,        // >
    Underline,  // (ins)
    Spoiler,    // (spl)
    Emoji,      // :
    EmjStart,   // (emj)
    EmjMiddle,  // (emj)[
    EmjEnd,     // ]
    Channel,    // (chn)
    Mention,    // (met)
    Role,       // (rol)
    InlineCode, // `
    Code,       // ```
}

const SPC_TABLE: &[u8; 13] = b"\n\r*~[])->(:`\\";
// [
//     b'\n', b'\r', b'*', b'~', b'[', b']', b')', b'-', b'>', b'(', b':', b'`', b'\\',
// ];
const ESCAPE_CHAR: &[u8; 13] = b"!()*-.:>[\\]`~";

pub(crate) fn parse_str(s: &str) -> Vec<Item> {
    FirstPass {
        text: s.as_bytes(),
        items: vec![],
    }
    .run()
}

struct FirstPass<'a> {
    text: &'a [u8],
    items: Vec<Item>,
}

impl<'a> FirstPass<'a> {
    fn run(mut self) -> Vec<Item> {
        let mut ix = 0;
        while ix < self.text.len() {
            ix = self.parse_block(ix);
        }
        self.items
    }
    fn parse_block(&mut self, ix: usize) -> usize {
        match self.text[ix] {
            b'\r' => self
                .push_and_return(ix, 4, b"\r\n\r\n", ItemBody::HardBreak)
                .or_else(|| self.push_and_return(ix, 2, b"\r\r", ItemBody::HardBreak))
                .or_else(|| self.push_and_return(ix, 2, b"\r\n", ItemBody::SoftBreak))
                .unwrap_or_else(|| self.push(ix, ItemBody::SoftBreak)),
            b'\n' => self
                .push_and_return(ix, 2, b"\n\n", ItemBody::HardBreak)
                .unwrap_or_else(|| self.push(ix, ItemBody::SoftBreak)),
            b'*' => self
                .push_and_return(ix, 2, b"**", ItemBody::Blod)
                .unwrap_or_else(|| self.push(ix, ItemBody::Italic)),
            b'~' => self
                .push_and_return(ix, 2, b"~~", ItemBody::Deleted)
                .unwrap_or_else(|| self.push_text(ix)),
            b'[' => self.push(ix, ItemBody::LinkStart),
            b']' => self
                .push_and_return(ix, 2, b"](", ItemBody::LinkMiddle)
                .unwrap_or_else(|| self.push(ix, ItemBody::EmjEnd)),
            b')' => self.push(ix, ItemBody::LinkEnd),
            b'-' => self
                .push_and_return(ix, 3, b"---", ItemBody::Divider)
                .unwrap_or_else(|| self.push_text(ix)),
            b'>' => self
                .push_and_return(ix, 2, b"> ", ItemBody::Ref)
                .unwrap_or_else(|| self.push_text(ix)),
            b'(' => self
                .push_and_return(ix, 6, b"(emj)[", ItemBody::EmjMiddle)
                .or_else(|| self.push_and_return(ix, 5, b"(ins)", ItemBody::Underline))
                .or_else(|| self.push_and_return(ix, 5, b"(spl)", ItemBody::Spoiler))
                .or_else(|| self.push_and_return(ix, 5, b"(emj)", ItemBody::EmjStart))
                .or_else(|| self.push_and_return(ix, 5, b"(chn)", ItemBody::Channel))
                .or_else(|| self.push_and_return(ix, 5, b"(met)", ItemBody::Mention))
                .or_else(|| self.push_and_return(ix, 5, b"(rol)", ItemBody::Role))
                .unwrap_or_else(|| self.push_text(ix)),
            b':' => self.push(ix, ItemBody::Emoji),
            b'`' => self
                .push_and_return(ix, 3, b"```", ItemBody::Code)
                .unwrap_or_else(|| self.push(ix, ItemBody::InlineCode)),
            b'\\' => {
                if self.text.len() >= ix + 1 {
                    if ESCAPE_CHAR.contains(&self.text[ix + 1]) {
                        self.items.push(Item {
                            start: ix + 1,
                            end: ix + 2,
                            body: ItemBody::Text,
                        })
                    } else {
                        self.items.push(Item {
                            start: ix,
                            end: ix + 2,
                            body: ItemBody::Text,
                        })
                    }
                    ix + 2
                } else {
                    self.items.push(Item {
                        start: ix,
                        end: ix + 1,
                        body: ItemBody::Text,
                    });
                    ix + 1
                }
            }
            _ => self.push_text(ix),
        }
    }
    fn push_and_return(
        &mut self,
        ix: usize,
        offset: usize,
        expect: &[u8],
        body: ItemBody,
    ) -> Option<usize> {
        if self.text.len() >= ix + offset {
            if &self.text[ix..ix + offset] == expect {
                self.items.push(Item {
                    start: ix,
                    end: ix + offset,
                    body,
                });
                return Some(ix + offset);
            }
        }
        None
    }
    fn push(&mut self, ix: usize, body: ItemBody) -> usize {
        self.items.push(Item {
            start: ix,
            end: ix + 1,
            body,
        });
        ix + 1
    }
    fn push_text(&mut self, ix: usize) -> usize {
        let mut end = ix + 1;
        while end < self.text.len() {
            if SPC_TABLE.contains(&self.text[end]) {
                break;
            } else {
                end += 1;
            }
        }
        self.items.push(Item {
            start: ix,
            end,
            body: ItemBody::Text,
        });
        end
    }
}

pub(crate) fn parse(text: &str, input: &Vec<Item>) -> Vec<KMDItem> {
    Parser {
        text,
        input,
        output: vec![],
        expect: ItemBody::Text,
    }
    .run()
}

struct Parser<'a> {
    text: &'a str,
    input: &'a Vec<Item>,
    output: Vec<KMDItem>,
    expect: ItemBody,
}

macro_rules! vec_sub {
    ($fname: ident, $item: tt) => {
        fn $fname(&mut self, ix: usize) -> usize {
            let parser = self.sub_parser(ItemBody::$item);
            match parser.expect_vec(ix + 1) {
                Some((v, ix)) => {
                    self.output.push(KMDItem::$item(v));
                    ix
                }
                None => {
                    self.push_text(ix);
                    ix + 1
                }
            }
        }
    };
}

macro_rules! str_sub {
    ($fname: ident, $item: tt) => {
        fn $fname(&mut self, ix: usize) -> usize {
            let mut parser = self.sub_parser(ItemBody::$item);
            if let Some(nix) = parser.expect_str(ix + 1) {
                self.output.push(KMDItem::$item(
                    self.text[self.input[ix + 1].start..self.input[nix - 1].end].to_owned(),
                ));
                nix + 1
            } else {
                ix + 1
            }
        }
    };
}

impl<'a> Parser<'a> {
    fn sub_parser(&self, expect: ItemBody) -> Self {
        Self {
            text: self.text,
            input: self.input,
            output: vec![],
            expect,
        }
    }
    fn run(mut self) -> Vec<KMDItem> {
        let mut ix = 0;
        while ix < self.input.len() {
            ix = self.parse_block(ix);
        }
        self.output
    }
    fn parse_block(&mut self, ix: usize) -> usize {
        match self.input[ix].body {
            ItemBody::Text => self.push_text(ix),
            ItemBody::Blod => self.push_bold(ix),
            ItemBody::Italic => self.push_italic(ix),
            ItemBody::Deleted => self.push_deleted(ix),
            ItemBody::LinkStart => self.push_link(ix),
            ItemBody::LinkMiddle => self.push_text(ix),
            ItemBody::LinkEnd => self.push_text(ix),
            ItemBody::Divider
                if (self.input[ix - 1].body == ItemBody::SoftBreak
                    || self.input[ix - 1].body == ItemBody::HardBreak)
                    && (self.input[ix + 1].body == ItemBody::SoftBreak
                        || self.input[ix + 1].body == ItemBody::HardBreak) =>
            {
                self.output.push(KMDItem::Divider);
                ix + 1
            }
            ItemBody::Divider => self.push_text(ix),
            ItemBody::Ref => self.push_ref(ix),
            ItemBody::Underline => self.push_underline(ix),
            ItemBody::Spoiler => self.push_spoiler(ix),
            ItemBody::Emoji => self.push_emoji(ix),
            ItemBody::EmjStart => self.push_emj(ix),
            ItemBody::EmjMiddle => self.push_text(ix),
            ItemBody::EmjEnd => self.push_text(ix),
            ItemBody::SoftBreak => {
                self.output.push(KMDItem::NewLine);
                ix + 1
            }
            ItemBody::HardBreak => {
                self.output.push(KMDItem::NewLine);
                self.output.push(KMDItem::NewLine);
                ix + 1
            }
            ItemBody::Channel => self.push_channel(ix),
            ItemBody::Mention => self.push_mention(ix),
            ItemBody::Role => self.push_role(ix),
            ItemBody::InlineCode => self.push_inline_code(ix),
            ItemBody::Code => self.push_code(ix),
        }
    }
    /// try to find the another part
    /// return inner vec and next index to parse
    fn expect_vec(mut self, ix: usize) -> Option<(Vec<KMDItem>, usize)> {
        let mut nix = ix;
        while nix < self.input.len() {
            match self.input[nix].body {
                ItemBody::Text => nix = self.push_text(nix),
                ItemBody::Blod if self.expect == ItemBody::Blod && nix != ix => {
                    return Some((self.output, nix + 1));
                }
                ItemBody::Blod if self.expect == ItemBody::Blod => break,
                ItemBody::Blod => nix = self.push_bold(nix),
                ItemBody::Italic if self.expect == ItemBody::Italic && nix != ix => {
                    return Some((self.output, nix + 1));
                }
                ItemBody::Italic if self.expect == ItemBody::Italic => break,
                ItemBody::Italic => nix = self.push_italic(nix),
                ItemBody::Deleted if self.expect == ItemBody::Deleted && nix != ix => {
                    return Some((self.output, nix + 1));
                }
                ItemBody::Deleted if self.expect == ItemBody::Deleted => break,
                ItemBody::Deleted => nix = self.push_deleted(nix),
                ItemBody::Underline if self.expect == ItemBody::Underline && nix != ix => {
                    return Some((self.output, nix + 1));
                }
                ItemBody::Underline if self.expect == ItemBody::Underline => break,
                ItemBody::Underline => nix = self.push_underline(nix),
                ItemBody::Spoiler if self.expect == ItemBody::Spoiler && nix != ix => {
                    return Some((self.output, nix + 1));
                }
                ItemBody::Spoiler if self.expect == ItemBody::Spoiler => break,
                ItemBody::Spoiler => nix = self.push_spoiler(nix),
                ItemBody::LinkStart => nix = self.push_link(nix),
                ItemBody::LinkMiddle => nix = self.push_text(nix),
                ItemBody::LinkEnd => nix = self.push_text(nix),
                ItemBody::Divider => nix = self.push_text(nix),
                ItemBody::Ref => nix = self.push_text(nix),
                ItemBody::Emoji => nix = self.push_emoji(nix),
                ItemBody::EmjStart => nix = self.push_emj(nix),
                ItemBody::EmjMiddle => nix = self.push_text(nix),
                ItemBody::EmjEnd => nix = self.push_text(nix),
                ItemBody::Channel => nix = self.push_channel(nix),
                ItemBody::Mention => nix = self.push_mention(nix),
                ItemBody::Role => nix = self.push_role(nix),
                ItemBody::InlineCode => nix = self.push_inline_code(nix),
                ItemBody::Code => nix = self.push_code(nix),
                ItemBody::SoftBreak if self.expect == ItemBody::Ref => {
                    self.output.push(KMDItem::NewLine);
                    nix += 1;
                }
                ItemBody::SoftBreak | ItemBody::HardBreak => {
                    break;
                }
            }
        }
        None
    }
    /// try to find the another part
    /// return current index
    fn expect_str(&mut self, ix: usize) -> Option<usize> {
        let mut nix = ix;
        while nix < self.input.len() {
            match self.input[nix].body {
                ItemBody::LinkMiddle if self.expect == ItemBody::LinkMiddle => return Some(nix),
                ItemBody::LinkEnd if self.expect == ItemBody::LinkEnd => return Some(nix),
                ItemBody::Emoji if self.expect == ItemBody::Emoji => return Some(nix),
                ItemBody::EmjMiddle if self.expect == ItemBody::EmjMiddle => return Some(nix),
                ItemBody::EmjEnd if self.expect == ItemBody::EmjEnd => return Some(nix),
                ItemBody::Channel if self.expect == ItemBody::Channel => return Some(nix),
                ItemBody::Mention if self.expect == ItemBody::Mention => return Some(nix),
                ItemBody::Role if self.expect == ItemBody::Role => return Some(nix),
                ItemBody::InlineCode if self.expect == ItemBody::InlineCode => return Some(nix),
                ItemBody::SoftBreak | ItemBody::HardBreak => break,
                _ => nix += 1,
            }
        }
        None
    }
    fn push_text(&mut self, ix: usize) -> usize {
        let Item { start, end, .. } = self.input[ix];
        self.output
            .push(KMDItem::Text(self.text[start..end].to_owned()));
        ix + 1
    }
    // fn push_texts(&mut self, start: usize, end: usize) -> usize {
    //     self.output.push(KMDItem::Text(
    //         self.text[self.input[start].start..self.input[end].end].to_owned(),
    //     ));
    //     end + 1
    // }
    fn push_bold(&mut self, ix: usize) -> usize {
        let parser = self.sub_parser(ItemBody::Blod);
        match parser.expect_vec(ix + 1) {
            Some((v, ix)) => {
                self.output.push(KMDItem::Blod(v));
                ix
            }
            None => {
                self.push_text(ix);
                ix + 1
            }
        }
    }
    vec_sub!(push_italic, Italic);
    vec_sub!(push_deleted, Deleted);
    vec_sub!(push_ref, Ref);
    vec_sub!(push_underline, Underline);
    vec_sub!(push_spoiler, Spoiler);
    fn push_link(&mut self, ix: usize) -> usize {
        let mut parser = self.sub_parser(ItemBody::LinkMiddle);
        if let Some(nix) = parser.expect_str(ix + 1) {
            let mut parser = self.sub_parser(ItemBody::LinkEnd);
            if let Some(nnix) = parser.expect_str(nix + 1) {
                self.output.push(KMDItem::Link {
                    text: self.text[self.input[ix + 1].start..self.input[nix - 1].end].to_owned(),
                    url: self.text[self.input[nix + 1].start..self.input[nnix - 1].end].to_owned(),
                });
                return nnix + 1;
            }
        }
        self.push_text(ix);
        ix + 1
    }
    fn push_emj(&mut self, ix: usize) -> usize {
        let mut parser = self.sub_parser(ItemBody::EmjMiddle);
        if let Some(nix) = parser.expect_str(ix + 1) {
            let mut parser = self.sub_parser(ItemBody::EmjEnd);
            if let Some(nnix) = parser.expect_str(nix + 1) {
                self.output.push(KMDItem::Emoji(
                    self.text[self.input[ix + 1].start..self.input[nix - 1].end].to_owned(),
                    Some(self.text[self.input[nix + 1].start..self.input[nnix - 1].end].to_owned()),
                ));
                return nnix + 1;
            }
        }
        self.push_text(ix);
        ix + 1
    }
    fn push_emoji(&mut self, ix: usize) -> usize {
        let mut parser = self.sub_parser(ItemBody::Emoji);
        if let Some(nix) = parser.expect_str(ix + 1) {
            self.output.push(KMDItem::Emoji(
                self.text[self.input[ix + 1].start..self.input[nix - 1].end].to_owned(),
                None,
            ));
            nix + 1
        } else {
            ix + 1
        }
    }
    fn push_channel(&mut self, ix: usize) -> usize {
        let mut parser = self.sub_parser(ItemBody::Channel);
        if let Some(nix) = parser.expect_str(ix + 1) {
            self.output.push(KMDItem::Channel(
                self.text[self.input[ix + 1].start..self.input[nix - 1].end].to_owned(),
            ));
            nix + 1
        } else {
            ix + 1
        }
    }
    str_sub!(push_mention, Mention);
    str_sub!(push_role, Role);
    str_sub!(push_inline_code, InlineCode);
    fn push_code(&mut self, ix: usize) -> usize {
        let mut nix = ix + 1;
        let (ty, six) = if let ItemBody::Text = self.input[nix].body {
            nix += 1;
            (
                self.text[self.input[nix].start..self.input[nix].end].to_owned(),
                nix,
            )
        } else {
            (String::default(), nix)
        };
        while nix < self.input.len() {
            match self.input[nix].body {
                ItemBody::Code => {
                    self.output.push(KMDItem::Code {
                        ty,
                        content: self.text[self.input[six].start..self.input[nix].end].to_owned(),
                    });
                    return nix + 1;
                }
                _ => {
                    nix += 1;
                }
            }
        }
        self.push_text(ix);
        ix + 1
    }
}

pub fn kmd_from_str(s: &str) -> Vec<KMDItem> {
    let input = parse_str(s);
    parse(s, &input)
}

#[test]
fn t() {
    let temp0 = r#"**官匹冲分挑战赛**
---
欢迎大家参与一月份举办的官匹冲分挑战赛。
本次挑战赛举办的目的是为了增加小伙伴们之间的默契程度，希望大家能在此次活动中结交更多的朋友。
    
**活动开始时间**
`2021年1月1日至2021年1月30日`
    
**活动规则**
此次活动仅限(ins)**AK**(ins)及以下段位的小伙伴们参与，请大家保持绿色游戏原则，不要使用~~第三方辅助工具~~或~~炸鱼~~影响其他游戏玩家体验。
挑战赛奖励将根据账号累计上升段位发放奖励。
    
**活动奖励**
第一名：(spl)游戏加速器年卡(spl)
第二名：(spl)游戏加速器季卡(spl)
第三名：(spl)游戏加速器月卡(spl)
参与奖：(spl)所有参与活动的小伙伴都将获得一个服务器角色(spl)"#;
    let kmds = kmd_from_str(temp0);
    println!("{:?}", kmds);
    println!(
        "{}",
        kmds.iter().map(|kmd| kmd.to_string()).collect::<String>()
    );
}

#[test]
fn err_test() {
    let temp0 = r#"(met)**youmet) /echo a"#;
    println!("{:?}", kmd_from_str(temp0));
}
