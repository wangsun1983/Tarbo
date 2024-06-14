use std::mem::size_of;

use crate::tools::stringhelper;

use super::message_id;

pub const RoomEventEnter:u32 = 1;
pub const RoomEventMsg:u32 = 2;
pub const RoomEventLeave:u32 = 3;
pub const RoomEventCreate:u32 = 4;
pub const RoomEventErase:u32 = 5;
pub const RoomEventList:u32 = 6;
pub const RoomEventListUsers:u32 = 7;

pub struct RoomMessage {
    event:u32,
    room_name:[u8;128],
    //RoomEventMsg:         content->message
    //RoomEventList:        content->user list
    content:[u8;512] ,
    who:[u8;128],
}

impl RoomMessage {
    pub fn from_bytes(bytes:&[u8])->(RoomMessage,usize) {
        if bytes.len() < size_of::<RoomMessage>() {
            panic!("data is too short")
        }

        let msg:RoomMessage = unsafe {
            std::ptr::read(bytes.as_ptr() as *const _)
        };
        
        let size = msg.len();

        (msg,size)
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let bytes:&[u8] = unsafe {
            let ptr  = self as *const RoomMessage as *const u8;
            std::slice::from_raw_parts(ptr,size_of::<RoomMessage>())
        };
        
        let mut data = Vec::<u8>::new();
        let id_bytes = message_id::Id_RoomMessage.to_le_bytes();
        data.extend_from_slice(&id_bytes);
        data.extend_from_slice(bytes);

        return data;
    }

    pub fn new(event:u32,who:&String,room:&String,msg:&String)->Self {
        let room_bytes = room.as_str().as_bytes();
        let mut room_dest_bytes:[u8;128] = [0;128];
        room_dest_bytes[0..room_bytes.len()].copy_from_slice(room_bytes);

        let msg_bytes = msg.as_str().as_bytes();
        let mut msg_dest_bytes:[u8;512] = [0;512];
        msg_dest_bytes[0..msg_bytes.len()].copy_from_slice(msg_bytes);

        let who_bytes = who.as_str().as_bytes();
        let mut who_dest_bytes:[u8;128] = [0;128];
        who_dest_bytes[0..who_bytes.len()].copy_from_slice(who_bytes);

        RoomMessage {
            event:event,
            room_name:room_dest_bytes,
            content:msg_dest_bytes,
            who:who_dest_bytes
        }
    }

    pub fn set_content(&mut self,content:&String) {
        let content_bytes = content.as_str().as_bytes();
        self.content.fill(0);
        self.content[0..content_bytes.len()].copy_from_slice(content_bytes); 
    }

    pub fn get_event(&self)->u32 {
        self.event
    }

    pub fn get_room_name(&self)->String {
        stringhelper::to_string(&self.room_name)
    }

    pub fn get_content(&self)->String {
        stringhelper::to_string(&self.content)
    }

    pub fn get_who(&self)->String {
        stringhelper::to_string(&self.who)
    }

    pub fn len(&self)->usize {
        size_of::<RoomMessage>()
    }
}