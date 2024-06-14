use std::mem::size_of;

use crate::tools::stringhelper;

use super::message_id;

pub struct BroadCastMessage {
    msg_from:[u8;128],
    msg_content:[u8;1024],
}

impl BroadCastMessage {
    pub fn from_bytes(bytes:&[u8])->(BroadCastMessage,usize) {
        if bytes.len() < size_of::<BroadCastMessage>() {
            panic!("data is too short")
        }

        let msg:BroadCastMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const BroadCastMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<BroadCastMessage>())
        };
        
        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_BroadCast.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new(from:&String,content:&String)->Self {
        let from_src_bytes = from.as_str().as_bytes();
        let mut from_dest_bytes:[u8;128] = [0;128];
        from_dest_bytes[0..from_src_bytes.len()].copy_from_slice(from_src_bytes);

        let content_src_bytes = content.as_str().as_bytes();
        let mut content_dest_bytes:[u8;1024] = [0;1024];
        content_dest_bytes[0..content_src_bytes.len()].copy_from_slice(content_src_bytes);

        BroadCastMessage {
            msg_from:from_dest_bytes,
            msg_content:content_dest_bytes,
        }
    }

    pub fn get_msg_from(&self)->String {
        stringhelper::to_string(&self.msg_from)
    }


    pub fn get_content(&self)->String {
        stringhelper::to_string(&self.msg_content)
    }

    pub fn len(&self)->usize {
        size_of::<BroadCastMessage>()
    }
}