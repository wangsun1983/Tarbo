use openssl::{pkey::Private,pkey::Public, rsa::{Padding, Rsa}};
use crate::io::filestream::{TarInputFileStream,TarFileOutputStream};
use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;

pub struct TarTransformer {
    m_priv_key:Option<Rsa<Private>>,
    m_pub_key:Option<Rsa<Public>>,
    m_padding:Padding
}

impl TarTransformer {
    pub fn new_with_pub_key(path:String)->Self {
        let mut rsa = TarTransformer {
            m_priv_key:None,
            m_pub_key:None,
            m_padding:Padding::PKCS1
        };
        let mut input_stream = TarInputFileStream::new(&TarFile::new(path));
        let data = input_stream.read_all();
        rsa.m_pub_key = Some(Rsa::public_key_from_pem(&data).unwrap());
        rsa
    }

    pub fn new_with_priv_key(path:String)->Self {
        let mut rsa = TarTransformer {
            m_priv_key:None,
            m_pub_key:None,
            m_padding:Padding::PKCS1
        };
        let mut input_stream = TarInputFileStream::new(&TarFile::new(path));
        let data = input_stream.read_all();
        rsa.m_priv_key = Some(Rsa::private_key_from_pem(&data).unwrap());
        rsa
    }

    pub fn gen_key(dir:String) {
        let rsa = Rsa::generate(2048).unwrap();
        let pub_key = rsa.public_key_to_pem().unwrap();
        let priv_key = rsa.private_key_to_pem().unwrap();
        //write to file
        let priv_file = String::from(&dir) + "/id_rsa";
        let mut priv_out_stream: TarFileOutputStream = TarFileOutputStream::new_truncate_stream(&TarFile::new(priv_file));
        priv_out_stream.write(&priv_key);

        let pub_file = String::from(&dir) + "/id_rsa.pub";
        let mut pub_out_stream = TarFileOutputStream::new_truncate_stream(&TarFile::new(pub_file));
        pub_out_stream.write(&pub_key);
    }

    pub fn encrypt_data(&self,data:&[u8]) ->Vec<u8> {
        let mut result = Vec::<u8>::new();
        if let Some(pub_key) = &self.m_pub_key {
            let buff_size = pub_key.size() as usize;
            let mut buf = vec![0;buff_size];
            let mut start:usize = 0;

            //if use Padding::PKCS1 =>encrypt length = key length - 11
            //if use Padding::OAEP  =>encrypt length = key length - 42
            let mut encrypt_len = 0;
            if self.m_padding == Padding::PKCS1 {
                encrypt_len = buff_size - 11;
            } else if self.m_padding == Padding::PKCS1_OAEP{
                encrypt_len = buff_size - 42
            }
            
            let mut end = start + encrypt_len;

            if end > data.len() {
                end = data.len();
            }

            loop {
                let ret = pub_key.public_encrypt(&data[start..end], &mut buf, self.m_padding);
                match ret {
                    Ok(size)=> {
                        result.extend_from_slice(&buf[0..size]);
                        start += encrypt_len;
                        end += encrypt_len;
                        if start >= data.len() {
                            break;
                        }

                        if end > data.len() {
                            end = data.len();
                        }
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        } else if let Some(priv_key) = &self.m_priv_key {
            let buff_size = priv_key.size() as usize;
            let mut buf = vec![0;buff_size];
            let mut start:usize = 0;

            //if use Padding::PKCS1 =>encrypt length = key length - 11
            //if use Padding::OAEP  =>encrypt length = key length - 42
            let mut encrypt_len = 0;
            if self.m_padding == Padding::PKCS1 {
                encrypt_len = buff_size - 11;
            } else if self.m_padding == Padding::PKCS1_OAEP{
                encrypt_len = buff_size - 42
            }
            
            let mut end = start + encrypt_len;

            if end > data.len() {
                end = data.len();
            }

            loop {
                let ret = priv_key.private_encrypt(&data[start..end], &mut buf, self.m_padding);
                match ret {
                    Ok(size)=> {
                        result.extend_from_slice(&buf[0..size]);
                        start += encrypt_len;
                        end += encrypt_len;
                        if start >= data.len() {
                            break;
                        }

                        if end > data.len() {
                            end = data.len();
                        }
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        result
    }

    pub fn encrypt_string(&self,content:&String)->Vec<u8> {
        let data = content.as_bytes();
        return self.encrypt_data(data);
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


    pub fn decrypt_data(&self,data:&[u8]) ->Vec<u8> {
        let mut result = Vec::<u8>::new();
        if let Some(pub_key) = &self.m_pub_key {
            let buff_size = pub_key.size() as usize;
            let mut buf = vec![0;buff_size];
            let mut start:usize = 0;
            let mut end:usize = data.len();

            //if use Padding::PKCS1 =>encrypt length = key length - 11
            //if use Padding::OAEP  =>encrypt length = key length - 42
            let mut encrypt_len = 0;
            if self.m_padding == Padding::PKCS1 {
                encrypt_len = buff_size - 11;
            } else if self.m_padding == Padding::PKCS1_OAEP{
                encrypt_len = buff_size - 42
            }

            if end > buff_size {
                end = buff_size;
            }

            loop {
                let ret = pub_key.public_decrypt(&data[start..end], &mut buf, Padding::PKCS1);
                match ret {
                    Ok(size)=> {
                        result.extend_from_slice(&buf[0..size]);
                        if size < encrypt_len {
                            break;
                        }

                        start += buff_size;
                        end += buff_size;

                        if end > data.len() {
                            end = data.len();
                        }
                        
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        } else if let Some(priv_key) = &self.m_priv_key {
            let buff_size = priv_key.size() as usize;
            let mut buf = vec![0;buff_size];
            let mut start:usize = 0;
            let mut end:usize = data.len();

            //if use Padding::PKCS1 =>encrypt length = key length - 11
            //if use Padding::OAEP  =>encrypt length = key length - 42
            let mut encrypt_len = 0;
            if self.m_padding == Padding::PKCS1 {
                encrypt_len = buff_size - 11;
            } else if self.m_padding == Padding::PKCS1_OAEP{
                encrypt_len = buff_size - 42
            }

            if end > buff_size {
                end = buff_size;
            }

            loop {
                let ret = priv_key.private_decrypt(&data[start..end], &mut buf, Padding::PKCS1);
                match ret {
                    Ok(size)=> {
                        result.extend_from_slice(&buf[0..size]);
                        if size < encrypt_len {
                            break;
                        }

                        start += buff_size;
                        end += buff_size;

                        if end > data.len() {
                            end = data.len();
                        }
                        
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        result
    }

    pub fn decrypt_string(&self,content:&String)->Vec<u8> {
        let data = content.as_bytes();
        return self.decrypt_data(data);
    }

    pub fn decrypt_file(&self,path:&String)->Vec<u8> {
        let mut stream = TarInputFileStream::new(&TarFile::new(String::from(path)));
        let file_data = stream.read_all();

        let ptr = file_data.as_ptr();
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(ptr,file_data.len())
        };

        return self.decrypt_data(data);
    }


}

