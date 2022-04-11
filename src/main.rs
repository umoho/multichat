mod send;
mod recv;
mod conf;
mod proto;
mod user;
mod cli;
mod data;
mod crypto;

use clap::StructOpt;
use colored::*;

fn main() {

    let args = cli::Args::parse();
    let config = conf::Config::from(args.room,
        args.user,
        conf::DEF_BROADCAST_ADDR.to_string(),
    args.port);
    recv::listener_thread(config);

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


