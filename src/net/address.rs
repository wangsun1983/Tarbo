pub struct TarAddress {
    address:String,
    port:u32,
}

impl TarAddress {
    pub fn new(address:&str,port:u32)->TarAddress {
        TarAddress {
            address:String::from(address),
            port:port
        }
    }

    pub fn get_address(&self)->String {
        String::from(&self.address)
    }

    pub fn get_port(&self)->u32 {
        self.port
    }

    pub fn to_string(&self)->String {
        let coton = String::from(":");
        let port_str = self.port.to_string();

        self.address.clone() + &coton + &port_str
    }
}