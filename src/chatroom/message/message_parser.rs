use crate::chatroom::message::message_id;
use crate::chatroom::message::new_comer_message::NewComerMessage;
use crate::chatroom::message::user_exit_message::UserExitMessage;
use crate::chatroom::message::chat_message::ChatMessage;
use crate::chatroom::message::common_ack_message::CommonAckMessage;
use crate::chatroom::message::chatter_list_message::ChatterListMessage;

use super::broadcast_message::BroadCastMessage;
use super::file_transfer_message::FileTransferMessage;
use super::room_message::RoomMessage;

pub struct MessageParser {
    
}

impl MessageParser {
    pub fn get_id_size(id:u32)->usize {
        match id {
            message_id::Id_NewComer => {
                return std::mem::size_of::<NewComerMessage>();
            },
            message_id::Id_UserExit => {
                return std::mem::size_of::<UserExitMessage>();
            },
            message_id::Id_ChatMessage => {
                return std::mem::size_of::<ChatMessage>();
            },
            message_id::Id_CommonAck => {
                return std::mem::size_of::<CommonAckMessage>();
            },
            message_id::Id_ChatterList => {
                return std::mem::size_of::<ChatterListMessage>();
            },
            message_id::Id_FileTransfer => {
                return std::mem::size_of::<FileTransferMessage>();
            },
            message_id::Id_BroadCast=> {
                return std::mem::size_of::<BroadCastMessage>();
            },
            message_id::Id_RoomMessage=> {
                return std::mem::size_of::<RoomMessage>();
            }
            _ => {
                return 0;
            }
        }
    }
}