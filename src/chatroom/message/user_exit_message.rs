use std::mem::size_of;

use crate::tools::stringhelper;

use super::message_id;

pub struct UserExitMessage {
    name:[u8;128],
}

impl UserExitMessage {
    pub fn from_bytes(bytes:&[u8])->(UserExitMessage,usize) {
        if bytes.len() < size_of::<UserExitMessage>() {
            panic!("data is too short")
        }


        let msg:UserExitMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const UserExitMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<UserExitMessage>())
        };

        //Vec::from(bytes)
        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_UserExit.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new(name:String)->Self {
        let name_src_bytes = name.as_str().as_bytes();
        let mut name_dest_bytes:[u8;128] = [0;128];
        name_dest_bytes[0..name_src_bytes.len()].copy_from_slice(name_src_bytes);

        UserExitMessage {
            name:name_dest_bytes
        }
    }

    pub fn get_name(&self)->String {
        stringhelper::to_string(&self.name)
    }
    
    pub fn len(&self)->usize {
        size_of::<UserExitMessage>()
    }
}