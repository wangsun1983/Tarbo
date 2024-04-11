use tokio::net::TcpStream;

use crate::net::address;

struct TarSocket {
    addr:address::TarAddress,

}

impl TarSocket {
    pub fn connect(&self) {
        let mut stream = 
            TcpStream::connect(self.addr.to_string());
    }
}