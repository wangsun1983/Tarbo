use openssl::md::Md;
use openssl::md_ctx::MdCtx;

use crate::io::filestream::TarInputFileStream;
use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;

pub struct TarTransformer {}

impl TarTransformer {
    pub fn new()->Self {
        TarTransformer {
        }
    }

    pub fn encrypt_data(data:&[u8]) ->String {
        let mut ctx = MdCtx::new().unwrap();
        ctx.digest_init(Md::md5());
        ctx.digest_update(data);
        let mut digest = [0;128];
        let result = ctx.digest_final(&mut digest).unwrap();
        return hex::encode(&digest[0..result]);
    }

    pub fn encrypt_string(content:&String)->String {
        let data = content.as_bytes();
        return TarTransformer::encrypt_data(data);
    }

    pub fn encrypt_file(path:&String)->String {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return TarTransformer::encrypt_data(data);
    }
}