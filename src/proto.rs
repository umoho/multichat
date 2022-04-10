
pub struct Protocol {
    pub json_content: String,
}

impl Protocol {
    pub fn new(chat_message: ChatMessage) -> Self {
        let json_content = build_json_content(chat_message);
        Protocol {
            json_content,
        }
    }
    pub fn as_buf(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.json_content.as_bytes());
        buf
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        match String::from_utf8(buf) {
            Ok(json_content) => Some(Protocol { json_content }),
            Err(_) => None,
        }
    }
}

pub struct ChatMessage {
    pub user_name: String,
    pub msg_content: String,
    pub msg_time: String,
}

impl ChatMessage {
    pub fn new(user_name: String, msg_content: String, msg_time: String) -> Self {
        ChatMessage {
            user_name,
            msg_content,
            msg_time,
        }
    }

    pub fn from_proto(proto: &Protocol) -> Option<Self> {
        let parsed = parse_json_content(proto.json_content.as_str());
        let content = match parsed {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                return None
            }
        };
        Some(content)
    }
}

fn build_json_content(chat_message: ChatMessage) -> String {
    let mut json_content = json::JsonValue::new_object();
    json_content["user-name"] = json::JsonValue::from(chat_message.user_name);
    json_content["msg-content"] = json::JsonValue::from(chat_message.msg_content);
    json_content["msg-time"] = json::JsonValue::from(chat_message.msg_time);
    let json_content_str = json_content.dump();
    json_content_str
}

fn parse_json_content(json_content: &str) -> Result<ChatMessage, json::JsonError> {
    let json_content_obj = json::parse(json_content)?;
    let user_name = json_content_obj["user-name"].as_str().unwrap();
    let msg_content = json_content_obj["msg-content"].as_str().unwrap();
    let msg_time = json_content_obj["msg-time"].as_str().unwrap();
    Ok(ChatMessage::new(
        user_name.to_string(),
        msg_content.to_string(),
        msg_time.to_string()))
}
