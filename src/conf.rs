
pub const DEF_BROADCAST_ADDR: &str = "255.255.255.255";

#[derive(Clone)]
pub struct Config {
    pub room_id: String,
    pub user_name: String,
    pub broadcast_addr: String,
    pub service_port: u16,
}

impl Config {
    pub fn from(room_id: String, user_name: String, broadcast_addr: String, service_port: u16)
        -> Self {
        Config {
            room_id,
            user_name,
            broadcast_addr,
            service_port,
        }
    }
}
