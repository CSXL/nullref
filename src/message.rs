//! Contains builders of all messages that can be sent between clients.
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{Serialize, SerializeStruct};
use serde_json::Result as SerdeResult;

#[derive(Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub message_type: String,
    pub body: String,
}

impl Message {
    pub fn new(from: String, to: String, message_type: String, body: String) -> Message {
        Message {
            from: from,
            to: to,
            message_type: message_type,
            body: body,
        }
    }

    pub fn to_json(&self) -> SerdeResult<String> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json: &str) -> SerdeResult<Message> {
        serde_json::from_str(json)
    }

    pub fn dump(&self) {
        println!("Message: {:?}", self);
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Message", 4)?;
        state.serialize_field("from", &self.from)?;
        state.serialize_field("to", &self.to)?;
        state.serialize_field("message_type", &self.message_type)?;
        state.serialize_field("body", &self.body)?;
        state.end()
    }
}

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Message")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut from = None;
        let mut to = None;
        let mut message_type = None;
        let mut body = None;
        while let Some(key) = map.next_key()? {
            match key {
                "from" => {
                    if from.is_some() {
                        return Err(serde::de::Error::duplicate_field("from"));
                    }
                    from = Some(map.next_value()?);
                }
                "to" => {
                    if to.is_some() {
                        return Err(serde::de::Error::duplicate_field("to"));
                    }
                    to = Some(map.next_value()?);
                }
                "message_type" => {
                    if message_type.is_some() {
                        return Err(serde::de::Error::duplicate_field("message_type"));
                    }
                    message_type = Some(map.next_value()?);
                }
                "body" => {
                    if body.is_some() {
                        return Err(serde::de::Error::duplicate_field("body"));
                    }
                    body = Some(map.next_value()?);
                }
                _ => {}
            }
        }
        let from = from.ok_or_else(|| serde::de::Error::missing_field("from"))?;
        let to = to.ok_or_else(|| serde::de::Error::missing_field("to"))?;
        let message_type = message_type.ok_or_else(|| serde::de::Error::missing_field("message_type"))?;
        let body = body.ok_or_else(|| serde::de::Error::missing_field("body"))?;
        Ok(Message {
            from,
            to,
            message_type,
            body,
        })
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MessageVisitor)
    }
}

pub mod types {
    pub const MESSAGE: &str = "message";
    pub const DISCOVERY: &str = "discovery";
    pub const ACK: &str = "ack";
    pub const ERROR: &str = "error";
    pub const DISCONNECT: &str = "disconnect";
}

pub mod errors {
    pub const INVALID_MESSAGE: &str = "Invalid message";
    pub const INVALID_DISCOVERY: &str = "Invalid discovery";
    pub const INVALID_ACK: &str = "Invalid ack";
    pub const INVALID_ERROR: &str = "Invalid error";
    pub const INVALID_DISCONNECT: &str = "Invalid disconnect";
}

pub fn build_message(from: String, to: String, body: String) -> String {
    Message::new(from, to, types::MESSAGE.to_string(), body)
        .to_json()
        .unwrap()
}

pub fn build_discovery(from: String, to: String, body: String) -> String {
    Message::new(from, to, types::DISCOVERY.to_string(), body)
        .to_json()
        .unwrap()
}

pub fn build_ack(from: String, to: String, body: String) -> String {
    Message::new(from, to, types::ACK.to_string(), body)
        .to_json()
        .unwrap()
}

pub fn build_error(from: String, to: String, body: String) -> String {
    Message::new(from, to, types::ERROR.to_string(), body)
        .to_json()
        .unwrap()
}

pub fn build_disconnect(from: String, to: String, body: String) -> String {
    Message::new(from, to, types::DISCONNECT.to_string(), body)
        .to_json()
        .unwrap()
}
