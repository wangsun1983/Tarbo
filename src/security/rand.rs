use openssl::rand::rand_bytes;

pub struct TarTransformer {
}

impl TarTransformer {
    pub fn rand_bytes(buf:&mut [u8]) {
        rand_bytes(buf);
    }
}