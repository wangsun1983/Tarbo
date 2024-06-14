use std::mem::size_of;

use crate::tools::stringhelper;

use super::message_id;

pub struct NewComerMessage {
    name:[u8;128],
    old_name:[u8;128],
}

impl NewComerMessage {
    pub fn from_bytes(bytes:&[u8])->(NewComerMessage,usize) {
        if bytes.len() < size_of::<NewComerMessage>() {
            panic!("data is too short")
        }


        let msg:NewComerMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const NewComerMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<NewComerMessage>())
        };

        //Vec::from(bytes)
        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_NewComer.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);
        return data;
    }

    pub fn new(name:String)->Self {
        let name_src_bytes = name.as_str().as_bytes();
        let mut name_dest_bytes:[u8;128] = [0;128];
        name_dest_bytes[..name_src_bytes.len()].copy_from_slice(name_src_bytes);

        NewComerMessage {
            name:name_dest_bytes,
            old_name:[0;128]
        }
    }

    pub fn new_update_name(new_name:String,old_name:String)->Self {
        let name_src_bytes = new_name.as_str().as_bytes();
        let mut name_dest_bytes:[u8;128] = [0;128];
        name_dest_bytes[..name_src_bytes.len()].copy_from_slice(name_src_bytes);

        let old_name_src_bytes = old_name.as_str().as_bytes();
        let mut old_name_dest_bytes:[u8;128] = [0;128];
        old_name_dest_bytes[..old_name_src_bytes.len()].copy_from_slice(old_name_src_bytes);

        NewComerMessage {
            name:name_dest_bytes,
            old_name:old_name_dest_bytes
        }
    }

    pub fn get_name(&self)->String {
        stringhelper::to_string(&self.name)
    }

    pub fn get_old_name(&self)->String {
        stringhelper::to_string(&self.old_name)
    }
    
    pub fn len(&self)->usize {
        size_of::<NewComerMessage>()
    }
}