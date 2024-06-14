use crate::io::stream;
use crate::io::file;
use crate::io::stream::TarIoStream;

use core::slice;
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::fs::OpenOptions;
use std::os::unix::fs::MetadataExt;

use super::file::TarFile;

//---- TInputFileStream ----
pub struct TarInputFileStream {
    file:File,
}

impl TarInputFileStream {
    pub fn new(file:&TarFile)->Self {
        let f = std::fs::File::open(file.get_absolute_patch().unwrap()).unwrap();
        TarInputFileStream {
            file:f,
        }
    }    
}

impl TarIoStream for TarInputFileStream {
    fn read(&mut self,buff:&mut [u8])-> io::Result<usize> {
        self.file.read(buff)
    }

    fn write(&mut self,_:&[u8])->io::Result<usize> {
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "unsupport method"))
    }

    fn seek_to(&mut self,index:u64) ->bool {
        match self.file.seek(io::SeekFrom::Start(index)) {
            Ok(pos) => {
                return pos <= self.file.metadata().unwrap().size();
            },
            Err(_) => {}
        }
        false
    }

    fn read_all(&mut self)-> Vec<u8> {
        let size = self.file.metadata().unwrap().size();
        let mut vec_data:Vec<u8> = vec![0;size as usize];
        let ptr = vec_data.as_mut_ptr();
        let data: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(ptr,size as usize)
        };
        let _ = self.read(data); 
        return vec_data;
    }
}


//---- TOutputFileStream ----
pub struct TarFileOutputStream {
    file:File,
}

impl TarFileOutputStream {
    pub fn new_append_stream(file:&TarFile)->Self {
        let f = OpenOptions::new()
                                    .append(true)
                                    .open(file.get_absolute_patch().unwrap()).unwrap();
        TarFileOutputStream {
            file:f,
        }
    }

    pub fn new_truncate_stream(file:&TarFile)->Self {
        let f = std::fs::File::create(file.get_absolute_patch().unwrap()).unwrap();
        TarFileOutputStream {
            file:f,
        }
    }
}

impl TarIoStream for TarFileOutputStream {
    fn read(&mut self,_:&mut [u8])-> io::Result<usize> {
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "unsupport method"))
    }

    fn write(&mut self,data:&[u8])->io::Result<usize> {
        self.file.write(data)
    }

    fn seek_to(&mut self,index:u64) ->bool {
        match self.file.seek(io::SeekFrom::Start(index)) {
            Ok(pos) => {
                return pos == index
            },
            Err(_) => {}
        }
        false
    }

    fn read_all(&mut self)->Vec<u8> {
        panic!("not support");
    }
}

