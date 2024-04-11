use crate::io::stream;
use crate::io::file;
use crate::io::stream::TarIoStream;

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
}


//---- TOutputFileStream ----
pub struct TarOutputStream {
    file:File,
}

impl TarOutputStream {
    pub fn new_append_stream(file:&TarFile)->Self {
        let f = OpenOptions::new()
                                    .append(true)
                                    .open(file.get_absolute_patch().unwrap()).unwrap();
        TarOutputStream {
            file:f,
        }
    }

    pub fn new_truncate_stream(file:&TarFile)->Self {
        let f = std::fs::File::create(file.get_absolute_patch().unwrap()).unwrap();
        TarOutputStream {
            file:f,
        }
    }
}

impl TarIoStream for TarOutputStream {
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
}

