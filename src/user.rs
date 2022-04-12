use crate::{proto, send, conf::Config, data, crypto};

pub fn broadcast_chat(config: &Config, message: String) -> Result<(), std::io::Error> {
    
    // build chat message struct
    let chat_msg = data::ChatMessage::new(
        config.room_id.clone(),
        config.user_name.clone(),
        time::now_timestamp(),
         message
        );
    
    let encrypted_chat_msg = crypto::encrypt(
        chat_msg.as_json().as_str(),
        config.room_id.as_str());
    
    // build proto with encrypted chat message
    let proto = proto::Protocol::new(
        &encrypted_chat_msg);
    
    let buf = proto.as_buf();

    send::send_to_broadcast(
        buf,
        format!("{}:{}", config.broadcast_addr, config.service_port).as_str()
    )?;
    Ok(())
}

pub mod time {
    
    pub fn now_timestamp() -> i64 {
        let now = chrono::Local::now();
        let now_timestamp = now.timestamp_millis();
        now_timestamp
    }

    pub fn convert_timestamp_to_time(timestamp: i64) -> String {
        let naive = chrono::NaiveDateTime::from_timestamp(timestamp / 1000, 0);
        let now_str = naive.format("%Y-%m-%d %H:%M:%S UTC").to_string();
        now_str
    }

    pub fn difference(timestamp: i64) -> i64 {
        now_timestamp() - timestamp
    }

}
