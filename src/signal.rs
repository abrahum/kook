use crate::prelude::*;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Signal {
    Event(Event, i32),
    Hello(HelloContent),
    Ping(i32),
    Pong,
    Resume(i32),
    Reconnect(ReconnectContent),
    ResumeAck(ResumeAckContent),
}

impl<const V: u8> crate::KHL<V> {
    pub(crate) fn new_ping(&self) -> Signal {
        use std::sync::atomic::Ordering;
        Signal::Ping(self.sn.load(Ordering::SeqCst))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloContent {
    pub code: i32,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconnectContent {
    pub code: i32,
    pub err: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeAckContent {
    pub session_id: String,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum SinalField {
    S,
    D,
    Sn,
}

struct SignalVistor;

impl<'de> Visitor<'de> for SignalVistor {
    type Value = Signal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("should be a signal")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error as DeError;

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DItem {
            Event(Event),
            Hello(HelloContent),
            Reconnect(ReconnectContent),
            ResumeAck(ResumeAckContent),
        }

        let mut s: Option<u8> = None;
        let mut d: Option<DItem> = None;
        let mut sn: Option<i32> = None;
        while let Some(key) = map.next_key()? {
            match key {
                SinalField::S => s = Some(map.next_value()?),
                SinalField::D => d = Some(map.next_value()?),
                SinalField::Sn => sn = Some(map.next_value()?),
            }
        }
        match s.ok_or(DeError::missing_field("s"))? {
            0 => {
                if let DItem::Event(event) = d.ok_or(DeError::missing_field("d"))? {
                    Ok(Signal::Event(
                        event,
                        sn.ok_or(DeError::missing_field("sn"))?,
                    ))
                } else {
                    Err(DeError::custom("d should be event"))
                }
            }
            1 => {
                if let DItem::Hello(hello) = d.ok_or(DeError::missing_field("d"))? {
                    Ok(Signal::Hello(hello))
                } else {
                    Err(DeError::custom("d should be hello"))
                }
            }
            2 => Ok(Signal::Ping(sn.ok_or(DeError::missing_field("sn"))?)),
            3 => Ok(Signal::Pong),
            4 => Ok(Signal::Resume(sn.ok_or(DeError::missing_field("sn"))?)),
            5 => {
                if let DItem::Reconnect(reconnect) = d.ok_or(DeError::missing_field("d"))? {
                    Ok(Signal::Reconnect(reconnect))
                } else {
                    Err(DeError::custom("d should be reconnect"))
                }
            }
            6 => {
                if let DItem::ResumeAck(resume_ack) = d.ok_or(DeError::missing_field("d"))? {
                    Ok(Signal::ResumeAck(resume_ack))
                } else {
                    Err(DeError::custom("d should be resume_ack"))
                }
            }
            u => Err(DeError::unknown_variant(
                &u.to_string(),
                &["0", "1", "2", "3", "4", "5", "6"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Signal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(SignalVistor)
    }
}

impl Serialize for Signal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        match self {
            Signal::Event(event, sn) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("s", &0)?;
                map.serialize_entry("d", &event)?;
                map.serialize_entry("sn", &sn)?;
                map.end()
            }
            Signal::Hello(hello) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("s", &1)?;
                map.serialize_entry("d", &hello)?;
                map.end()
            }
            Signal::Ping(sn) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("s", &2)?;
                map.serialize_entry("sn", &sn)?;
                map.end()
            }
            Signal::Pong => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("s", &3)?;
                map.end()
            }
            Signal::Resume(sn) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("s", &4)?;
                map.serialize_entry("sn", &sn)?;
                map.end()
            }
            Signal::Reconnect(reconnect) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("s", &5)?;
                map.serialize_entry("d", &reconnect)?;
                map.end()
            }
            Signal::ResumeAck(resume_ack) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("s", &6)?;
                map.serialize_entry("d", &resume_ack)?;
                map.end()
            }
        }
    }
}

#[test]
fn de_test() {
    let datas = vec![
        r#"{
            "s": 1,
            "d": {
                "code": 0,
                "session_id": "xxxx"
            }
        }"#,
        r#"{
            "s": 2,
            "sn": 6
        }"#,
        r#"{
            "s": 3
        }"#,
        r#"{
            "s": 4,
            "sn": 100
        }"#,
        r#"{
            "s": 5,
            "d": {
                "code": 41008,
                "err": "Missing params"
            }
        }"#,
        r#"{
            "s": 6,
            "d": {
                "session_id": "xxxx-xxxxxx-xxx-xxx"
            }
        }"#,
    ];
    for data in datas {
        let signal: Signal = serde_json::from_str(data).unwrap();
        println!("{:?}", signal);
    }
}
