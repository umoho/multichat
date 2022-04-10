use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

    /// Your name
    #[clap(short, long)]
    pub user: String,

    /// The room id
    #[clap(short, long, default_value = "global")]
    pub room: String,

    /// The port of the service
    #[clap(short, long, default_value_t = 23380)]
    pub port: u16,
}
