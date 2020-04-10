struct Client{
    IP: String,
    PORT: String,


impl Client {
    fn tcp_client(self&) {
        use std::net::TcpStream;

        let mut conn = TcpStream::connect(Client:IP + Cleint:PORT);

        match conn {
            Ok(conn) => {
                println!("connected");
            }
            Err(e) => {
                println!("connection failed");
            }
        }
    }
}