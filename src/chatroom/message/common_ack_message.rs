use std::mem::size_of;
use super::message_id;
pub struct CommonAckMessage {
    id:u32,
    result:u32,
}

impl CommonAckMessage {
    pub fn from_bytes(bytes:&[u8])->(CommonAckMessage,usize) {
        if bytes.len() < size_of::<CommonAckMessage>() {
            panic!("data is too short")
        }


        let msg:CommonAckMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const CommonAckMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<CommonAckMessage>())
        };

        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_CommonAck.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new(id:u32,result:u32)->Self {
        CommonAckMessage {
            id:id,
            result:result
        }
    }

    pub fn set_result(&mut self,ret:u32) {
        self.result = ret;
    }

    pub fn get_result(&self)->u32 {
        return self.result;
    }

    pub fn get_id(&self)->u32 {
        return self.id;
    }
    
    pub fn len(&self)->usize {
        size_of::<CommonAckMessage>()
    }
}