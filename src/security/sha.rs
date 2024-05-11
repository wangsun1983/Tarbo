use openssl::sha;

use crate::io::{file::TarFile, filestream::TarInputFileStream, stream::TarIoStream};

pub struct Tar256Transformer{}

pub struct Tar224Transformer {}

pub struct Tar384Transformer {}

pub struct Tar512Transformer {}

impl Tar256Transformer {
    pub fn new()->Self {
        Tar256Transformer {
        }
    }

    pub fn encrypt_data(data:&[u8]) ->String {
        let mut sha256 = sha::Sha256::new();
        sha256.update(data);
        let sha = sha256.finish();
        return hex::encode(sha);
    }

    pub fn encrypt_string(content:&String)->String {
        let data = content.as_bytes();
        return Tar256Transformer::encrypt_data(data);
    }

    pub fn encrypt_file(path:&String)->String {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return Tar256Transformer::encrypt_data(data);
    }
}

impl Tar224Transformer {
    pub fn new()->Self {
        Tar224Transformer {
        }
    }

    pub fn encrypt_data(data:&[u8]) ->String {
        let mut sha224 = sha::Sha224::new();
        sha224.update(data);
        let sha = sha224.finish();
        return hex::encode(sha);
    }

    pub fn encrypt_string(content:&String)->String {
        let data = content.as_bytes();
        return Tar224Transformer::encrypt_data(data);
    }

    pub fn encrypt_file(path:&String)->String {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return Tar224Transformer::encrypt_data(data);
    }
}

impl Tar384Transformer {
    pub fn new()->Self {
        Tar384Transformer {
        }
    }

    pub fn encrypt_data(data:&[u8]) ->String {
        let mut sha384 = sha::Sha384::new();
        sha384.update(data);
        let sha = sha384.finish();
        return hex::encode(sha);
    }

    pub fn encrypt_string(content:&String)->String {
        let data = content.as_bytes();
        return Tar384Transformer::encrypt_data(data);
    }

    pub fn encrypt_file(path:&String)->String {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return Tar384Transformer::encrypt_data(data);
    }
}

impl Tar512Transformer {
    pub fn new()->Self {
        Tar512Transformer {
        }
    }

    pub fn encrypt_data(data:&[u8]) ->String {
        let mut sha512 = sha::Sha512::new();
        sha512.update(data);
        let sha = sha512.finish();
        return hex::encode(sha);
    }

    pub fn encrypt_string(content:&String)->String {
        let data = content.as_bytes();
        return Tar512Transformer::encrypt_data(data);
    }

    pub fn encrypt_file(path:&String)->String {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return Tar512Transformer::encrypt_data(data);
    }
}