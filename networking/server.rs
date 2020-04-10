struct Server{
    IP: String,
    PORT: String,
}


impl Server {

    fn tcp_server(self&) {
        use std::net::TcpListener;
 
        let listener = TcpListener::bind(Server::IP + Server::PORT).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("new client");
                }
                Err(e) => {
                    println!("connection fail");
                }
            }
        }
    }
}   