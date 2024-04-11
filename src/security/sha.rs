
use sha2::{Digest, Sha256};
use hex;

struct TarSha256{}

impl TarSha256 {
    fn encrypt(data:&[u8]) ->String {
        let hash = sha2::Sha256::digest(data);
        hex::encode(hash)
    }
}