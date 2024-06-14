use std::mem::size_of;
use crate::{chatroom::message::message_id, tools::stringhelper};
use crate::log;
pub struct ChatterListMessage {
    chatters:[u8;16*256] //all user's name(string)
}

impl ChatterListMessage {
    pub fn from_bytes(bytes:&[u8])->Vec<String> {
        if bytes.len() < size_of::<ChatterListMessage>() {
            panic!("data is too short")
        }


        let msg:ChatterListMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        //we should change to string
        let user_strings = stringhelper::to_string(&msg.chatters);
        let users = user_strings.split(",");
        let mut result = Vec::<String>::new();

        for user in users {
            result.push(user.to_string());
        }

        result
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const ChatterListMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<ChatterListMessage>())
        };

        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_ChatterList.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new_with_list(users:&Vec<String>)->Self {
        let mut users_string = String::from("");

        for user in users {
            log!("user is {}",user);
            users_string = users_string + &user + ",";
        }

        let final_string = &users_string[0..users_string.len() - 1];
        let mut msg = ChatterListMessage {
            chatters:[0;16*256]
        };

        msg.chatters[0..final_string.len()].clone_from_slice(final_string.as_bytes());
        return msg;
    }

    pub fn len(&self)->usize {
        size_of::<ChatterListMessage>()
    }

}