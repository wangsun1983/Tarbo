use std::mem::size_of;
use crate::tools::stringhelper;

use super::message_id;
pub struct FileTransferMessage {
    from:[u8;128],
    to:[u8;128],
    file_name:[u8;128],
    source_ip:[u8;64],
    port:u32,
    size:u32
}

impl FileTransferMessage {
    pub fn from_bytes(bytes:&[u8])->(FileTransferMessage,usize) {
        if bytes.len() < size_of::<FileTransferMessage>() {
            panic!("data is too short")
        }


        let msg:FileTransferMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const FileTransferMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<FileTransferMessage>())
        };

        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_FileTransfer.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new(to:&String,filename:&String,port:u32,size:u32)->Self {
        let mut msg = FileTransferMessage {
            from:[0;128],
            to:[0;128],
            file_name:[0;128],
            source_ip:[0;64],
            port:port,
            size:size
        };

        msg.to[0..to.len()].copy_from_slice(to.as_bytes());
        msg.file_name[0..filename.len()].copy_from_slice(filename.as_bytes());
        msg
    }

    pub fn get_name(&self)->String {
        stringhelper::to_string(&self.file_name)
    }

    pub fn get_from(&self)->String {
        stringhelper::to_string(&self.from)
    }

    pub fn get_to(&self)->String {
        stringhelper::to_string(&self.to) 
    }

    pub fn get_port(&self)->u32 {
        self.port
    }

    pub fn get_ip(&self) ->String {
        stringhelper::to_string(&self.source_ip)
    }

    pub fn set_ip(&mut self,ip:String) {
        self.source_ip[0..].fill(0);
        self.source_ip[0..ip.len()].copy_from_slice(ip.as_bytes());
    }

    pub fn set_from(&mut self,name:String) {
        self.from[0..].fill(0);
        self.from[0..name.len()].copy_from_slice(name.as_bytes());
    }
    
    pub fn get_file_size(&self)->u32 {
        self.size
    }

    pub fn len(&self)->usize {
        size_of::<FileTransferMessage>()
    }
}