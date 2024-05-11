use openssl::base64::decode_block;
use openssl::base64::encode_block;

pub struct TarTransformer {}

impl TarTransformer {
    pub fn encode(src:&[u8])->String {
        encode_block(src)
    }

    pub fn decode(src:&str)->Vec<u8> {
        decode_block(src).unwrap()
    }
}