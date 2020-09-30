use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self{ addr }
    }

    pub fn run(self) {
        println!("Server is listening {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("Ok");
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}