mod parse;

pub use parse::kmd_from_str;

#[derive(Debug)]
pub enum KMDItem {
    Text(String),
    Blod(Vec<KMDItem>),
    Italic(Vec<KMDItem>),
    Deleted(Vec<KMDItem>),
    Link { text: String, url: String },
    Divider,
    Ref(Vec<KMDItem>),
    Underline(Vec<KMDItem>),
    Spoiler(Vec<KMDItem>),
    Emoji(String, Option<String>),
    Channel(String),
    Mention(String),
    Role(String),
    InlineCode(String),
    Code { ty: String, content: String },
    // Escaped(String),
    NewLine,
}

fn v2s(v: &Vec<KMDItem>) -> String {
    v.iter().map(ToString::to_string).collect()
}

impl ToString for KMDItem {
    fn to_string(&self) -> String {
        match self {
            Self::Text(s) => s.to_string(),
            Self::Blod(v) => format!("**{}**", v2s(v)),
            Self::Italic(v) => format!("*{}*", v2s(v)),
            Self::Deleted(v) => format!("~~{}~~", v2s(v)),
            Self::Link { text, url } => format!("[{text}]({url})"),
            Self::Divider => "---".to_owned(),
            Self::Ref(v) => format!("> {}", v2s(v)),
            Self::Underline(v) => format!("(ins){}(ins)", v2s(v)),
            Self::Spoiler(v) => format!("(spl){}(spl)", v2s(v)),
            Self::Emoji(s, None) => format!(":{s}:"),
            Self::Emoji(s, Some(id)) => format!("(emj){s}(emj)[{id}]"),
            Self::Channel(s) => format!("(chn){s}(chn)"),
            Self::Mention(s) => format!("(met){s}(met)"),
            Self::Role(s) => format!("(rol){s}(rol)"),
            Self::InlineCode(s) => format!("`{s}`"),
            Self::Code { ty, content } => format!("```{ty}\n{content}```"),
            Self::NewLine => "\n".to_owned(),
        }
    }
}

impl KMDItem {
    pub fn plain_text(&self) -> String {
        match self {
            Self::Text(s) => s.to_string(),
            Self::Blod(v) => v2s(v),
            Self::Italic(v) => v2s(v),
            Self::Deleted(v) => v2s(v),
            Self::Link { text, .. } => format!("[{text}]"),
            Self::Divider => "---".to_owned(),
            Self::Ref(v) => v2s(v),
            Self::Underline(v) => v2s(v),
            Self::Spoiler(v) => v2s(v),
            Self::Emoji(s, ..) => s.clone(),
            Self::Channel(s) => s.clone(),
            Self::Mention(s) => s.clone(),
            Self::Role(s) => s.clone(),
            Self::InlineCode(s) => s.clone(),
            Self::Code { ty, content } => format!("{ty}\n{content}"),
            Self::NewLine => "\n".to_owned(),
        }
    }
}
