use crate::{proto, send, conf::Config};

pub fn broadcast_chat(config: &Config, message: String) -> Result<(), std::io::Error> {
    let chat_msg = proto::ChatMessage::new(
        config.user_name.clone(),
         message,
        now_time());
    let proto = proto::Protocol::new(chat_msg);
    let buf = proto.as_buf();

    send::send_to_broadcast(
        buf,
        format!("{}:{}", config.broadcast_addr, config.service_port).as_str()
    )?;
    Ok(())
}

fn now_time() -> String {
    let now = chrono::Local::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    now_str
}
