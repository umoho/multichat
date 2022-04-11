use crate::proto::Protocol;

const ROOM_ID: &str = "room-id";
const USER_NAME: &str = "user-name";
const MSG_TIME: &str = "msg-time";
const MSG_CONTENT: &str = "msg-content";

pub struct ChatMessage {
    pub room_id: String,
    pub user_name: String,
    pub msg_time: i64,
    pub msg_content: String,
}

impl ChatMessage {
    pub fn new(room_id: String, user_name: String, msg_time: i64, msg_content: String) -> Self {
        ChatMessage {
            room_id,
            user_name,
            msg_time,
            msg_content,
        }
    }

    pub fn from_proto(proto: &Protocol) -> Option<Self> {
        let parsed = parse_content_json(proto.as_json().as_str());
        let chat_message_content = match parsed {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                return None
            }
        };
        Some(chat_message_content)
    }

    pub fn as_json(&self) -> String {
        let mut json = json::JsonValue::new_object();
        json[ROOM_ID] = json::JsonValue::from(self.room_id.as_str());
        json[USER_NAME] = json::JsonValue::from(self.user_name.clone());
        json[MSG_TIME] = json::JsonValue::from(self.msg_time.clone());
        json[MSG_CONTENT] = json::JsonValue::from(self.msg_content.clone());
        json.dump()
    }
}

fn parse_content_json(content: &str) -> Result<ChatMessage, json::JsonError> {
    let content_obj = json::parse(content)?;
    let room_id = content_obj[ROOM_ID].as_str().unwrap();
    let user_name = content_obj[USER_NAME].as_str().unwrap();
    let msg_time = content_obj[MSG_TIME].as_i64().unwrap();
    let msg_content = content_obj[MSG_CONTENT].as_str().unwrap();
    Ok(ChatMessage::new(
        room_id.to_string(),
        user_name.to_string(),
        msg_time,
        msg_content.to_string()
        )
    )
}
