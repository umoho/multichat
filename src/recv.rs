use std::net::UdpSocket;
use colored::*;

use crate::{conf::Config, proto, data, crypto};

const MAX_BUF_SIZE: usize = 0x4000;

pub fn receive_from_broadcast(bind_port: u16)
        -> Result<(Vec<u8> ,(usize, std::net::SocketAddr)), std::io::Error> {
    let socket = UdpSocket::bind(
        format!("{}:{}", "0.0.0.0", bind_port).as_str()
    )?;

    let mut buf = [0; MAX_BUF_SIZE];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let buf_vec = buf[..amt].to_vec();

    Ok((buf_vec, (amt, src)))
}

pub fn listener_thread(config: Config) {
    std::thread::spawn(move || {
        loop {
            let port = &config.service_port;
            let (buff, (_amt, src)) 
                = receive_from_broadcast(
                    *port
                ).unwrap();

            let p = proto::Protocol::from_buf(buff);
            if let Some(p) = p {
                let decrypted_data = match crypto::decrypt(
                    p.get_data_value().unwrap().as_str(),
                    config.room_id.as_str()) 
                {
                    Some(d) => d,
                    None => {
                        println!("{}", "Failed to decrypt data".red());
                        continue;
                    },
                };

                let decrypted_protocol = proto::Protocol::from_json(
                    &decrypted_data).unwrap();
                
                if let Some(chat_msg) = data::ChatMessage::from_proto(&decrypted_protocol) {
                    println!("{}:\n{}\n", 
                        format!("{} {}",
                            chat_msg.user_name.green().bold(),
                            format!("({}) [{} ({})]",
                                src.ip().to_string().blue(),
                                crate::user::time::convert_timestamp_to_time(chat_msg.msg_time).red(),
                                format!("{}ms", crate::user::time::difference(chat_msg.msg_time).to_string()).red()
                            ),
                        ),
                        format!("<{}> {}", chat_msg.room_id.purple(), chat_msg.msg_content.yellow()),
                    );
                }
            }
        }
    });
}
