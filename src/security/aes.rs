use crate::io::filestream::TarInputFileStream;
use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;
use crate::io::filestream::TarFileOutputStream;

use openssl::aes::AesKey;
use openssl::symm::{self, Cipher};
pub struct TarTransformer {
    m_key:[u8;16],
    m_cipher:Cipher
}

enum TarTransformerPattern {
    ECB_128 = 0,
    CBC_128,
    XTS_128,
    CTR_128,
    CFB1_128,
    CFB128_128,
    CFB8_128,
    GCM_128,
    CCM_128,
    OFB_128,
    OCB_128,
    ECB_192,
    CBC_192,
    CTR_192,
    CFB1_192,
    CFB128_192,
    CFB8_192,
    GCM_192,
    CCM_192,
    OFB_192,
    OCB_192,
    ECB_256,
    CBC_256,
    XTS_256,
    CTR_256,
    CFB1_256,
    CFB128_256,
    CFB8_256,
    GCM_256,
    CCM_256,
    OFB_256,
    OCB_256
}

impl TarTransformer {
    pub fn gen_key(key_data:&[u8],path:String) {
        let len = key_data.len();
        let mut out_stream = TarFileOutputStream::new_truncate_stream(&TarFile::new(path));
        if len != 16 {
            if key_data.len() > 16 {
                let _ = out_stream.write(&key_data[0..16]);
            } else {
                let mut new_key_data:[u8;16] = [0;16];
                let mut index = 0;
                for v in key_data {
                    new_key_data[index] = *v;
                    index += 1;
                }
                out_stream.write(&new_key_data);
            }
        } else {
            out_stream.write(key_data);
        }
    }

    pub fn new_with_default(path:String)->Self {
        TarTransformer::new_with_pattern(TarTransformerPattern::CBC_128,path)
    }

    pub fn new_with_pattern(pattern:TarTransformerPattern,path:String)->Self {
        let mut stream = TarInputFileStream::new(&TarFile::new(path));
        let key_data = stream.read_all();
        let mut key:[u8;16] = [0;16];
        let mut count = 0;
        for v in &key_data {
            key[count] = *v;
            count += 1;
        }

        match pattern {
            TarTransformerPattern::ECB_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_cbc()
                }
            },
            TarTransformerPattern::CBC_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_cbc()
                }
            },
            TarTransformerPattern::XTS_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_xts()
                }
            },
            TarTransformerPattern::CTR_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_ctr()
                }
            },
            TarTransformerPattern::CFB1_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_cfb1()
                }
            },
            TarTransformerPattern::CFB128_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_cfb128()
                }
            },
            TarTransformerPattern::CFB8_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_cfb8()
                }
            },
            TarTransformerPattern::GCM_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_gcm()
                }
            },
            TarTransformerPattern::CCM_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_ccm()
                }
            },
            TarTransformerPattern::OFB_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_ofb()
                }
            },
            TarTransformerPattern::OCB_128 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_128_ocb()
                }
            },
            TarTransformerPattern::ECB_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_ecb()
                }
            },
            TarTransformerPattern::CBC_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_cbc()
                }
            },
            TarTransformerPattern::CTR_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_ctr()
                }
            },
            TarTransformerPattern::CFB1_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_cfb1()
                }
            },
            TarTransformerPattern::CFB128_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_cfb128()
                }
            },
            TarTransformerPattern::CFB8_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_cfb8()
                }
            },
            TarTransformerPattern::GCM_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_gcm()
                }
            },
            TarTransformerPattern::CCM_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_ccm()
                }
            },
            TarTransformerPattern::OFB_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_ofb()
                }
            },
            TarTransformerPattern::OCB_192 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_192_ocb()
                }
            },
            TarTransformerPattern::ECB_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_ecb()
                }
            },
            TarTransformerPattern::CBC_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_cbc()
                }
            },
            TarTransformerPattern::XTS_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_xts()
                }
            },
            TarTransformerPattern::CTR_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_ctr()
                }
            },
            TarTransformerPattern::CFB1_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_cfb1()
                }
            },
            TarTransformerPattern::CFB128_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_cfb128()
                }
            },
            TarTransformerPattern::CFB8_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_cfb8()
                }
            },
            TarTransformerPattern::GCM_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_gcm()
                }
            },
            TarTransformerPattern::CCM_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_ccm()
                }
            },
            TarTransformerPattern::OFB_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_ofb()
                }
            },
            TarTransformerPattern::OCB_256 =>{
                TarTransformer {
                    m_key:key,
                    m_cipher:Cipher::aes_256_ocb()
                }
            }
        }
    }

    pub fn encrypt_data(&self,data:&[u8])->Vec<u8> {
        let mut iv = [0;32];
        symm::encrypt(self.m_cipher, &self.m_key, Some(&iv), data).unwrap()
    }

    pub fn encrypt_file(&self,path:&String)->Vec<u8> {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return self.encrypt_data(data);
    }

    pub fn decrypt_data(&self,data:&[u8])->Vec<u8> {
        let mut iv = [0;32];
        symm::decrypt(self.m_cipher, &self.m_key, Some(&iv), data).unwrap()
    }
}