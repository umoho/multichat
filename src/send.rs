use std::net::UdpSocket;



pub fn send_to_broadcast(send_buf: Vec<u8>, broadcast_addr: &str)
        -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&send_buf, broadcast_addr).unwrap();
    Ok(())
}
