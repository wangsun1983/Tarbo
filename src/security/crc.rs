
use crc::Crc;

use crate::io::filestream::TarInputFileStream;
use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;

pub struct TarTransformer {

}

impl TarTransformer {
    pub fn crc32_data(data:&[u8])->u32 {
        let mut x25: Crc<u32> =  crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
        x25.checksum(data)
    }

    pub fn crc32_string(content:&String)->u32 {
        let data = content.as_bytes();
        return TarTransformer::crc32_data(data);
    }

    pub fn crc32_file(path:&String)->u32 {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return TarTransformer::crc32_data(data);
    }

    pub fn crc64_data(data:&[u8])->u64 {
        let mut x25: Crc<u64> =  crc::Crc::<u64>::new(&crc::CRC_64_MS);
        x25.checksum(data)
    }

    pub fn crc64_string(content:&String)->u64 {
        let data = content.as_bytes();
        return TarTransformer::crc64_data(data);
    }

    pub fn crc64_file(path:&String)->u64 {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return TarTransformer::crc64_data(data);
    }
}
