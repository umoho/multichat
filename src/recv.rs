use std::net::UdpSocket;



pub fn receive_from_broadcast(bind_port: u16)
        -> Result<(Vec<u8> ,(usize, std::net::SocketAddr)), std::io::Error> {
    let socket = UdpSocket::bind(
        format!("{}:{}", "0.0.0.0", bind_port).as_str()
    )?;

    let mut buf = [0; 8192];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let buf_vec = buf[..amt].to_vec();

    Ok((buf_vec, (amt, src)))
}
