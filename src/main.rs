mod send;
mod recv;
mod conf;
mod proto;
mod user;
mod cli;

use clap::StructOpt;
use colored::*;

fn main() {

    let args = cli::Args::parse();
    let config = conf::Config::from(args.room,
        args.user,
        conf::DEF_BROADCAST_ADDR.to_string(),
    args.port);
    
    std::thread::spawn(move || {
        loop {
            let port = &config.service_port;
            let (buff, (_amt, src)) 
                = recv::receive_from_broadcast(
                    *port
                ).unwrap();

            let p = proto::Protocol::from_buf(buff);
            if let Some(p) = p {
                if let Some(chat_msg) = proto::ChatMessage::from_proto(&p) {
                    println!("{}:\n{}\n", 
                        format!("{} {}",
                            chat_msg.user_name.green().bold(),
                            format!("({}) [{}]",
                                src.ip().to_string().blue(),
                                chat_msg.msg_time.red()),
                        ),
                        format!("<{}> {}", &config.room_id.purple(), chat_msg.msg_content.yellow()),
                    );
                }
            }
        }
    });

    let args = cli::Args::parse();
    let conf = &conf::Config::from(args.room,
        args.user,
        conf::DEF_BROADCAST_ADDR.to_string(),
    args.port);
    println!("{}", "Please input your chat message at following".yellow());
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.len() == 0 {
            continue;
        }
        user::broadcast_chat(conf, input.to_string()).unwrap();
    }
}


