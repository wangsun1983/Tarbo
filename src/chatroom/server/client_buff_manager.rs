use std::collections::HashMap;

use crate::lang::TarByteRingArray;
use crate::concurrent::TarMutex;
use crate::chatroom::message::message_parser;

use crate::log;
pub struct ClientBuffInfo {
    ring_buff:TarByteRingArray,
    current_len:usize,
    current_id:u32,
}

impl ClientBuffInfo {
    pub fn new()->Self {
        ClientBuffInfo {
            ring_buff:TarByteRingArray::new(),
            current_len:0,
            current_id:0
        }
    }
}

pub struct ClientBuffManager {
    m_client_buffs:HashMap<String,ClientBuffInfo>,    
    m_client_buffs_mutex:TarMutex 
}

impl ClientBuffManager {
    pub fn new()->Self {
        ClientBuffManager {
            m_client_buffs:HashMap::new(),
            m_client_buffs_mutex:TarMutex::new()
        }
    }

    pub fn add_new_client(&mut self,tag:&String) {
        let client_buf_info = ClientBuffInfo::new();
        self.m_client_buffs_mutex.lock();
        if self.m_client_buffs.contains_key(tag) {
            log!("ClientBuffManager: client already exists");
        }

        self.m_client_buffs.insert(tag.clone(),client_buf_info);
        self.m_client_buffs_mutex.unlock();
    }

    pub fn remove_client(&mut self,tag:&String) {
        self.m_client_buffs_mutex.lock();
        self.m_client_buffs.remove(tag);
        self.m_client_buffs_mutex.unlock();
    }

    pub fn push_buff(&mut self,tag:&String,data:&Vec<u8>) {
        self.m_client_buffs_mutex.lock();
        let client = self.m_client_buffs.get_mut(tag).unwrap();
        client.ring_buff.push(data.as_slice());
        self.m_client_buffs_mutex.unlock();
    }

    pub fn process_buff(&mut self,tag:&String,mut process_func:impl FnMut(u32,Vec<u8>)) {
        self.m_client_buffs_mutex.lock();
        let client = self.m_client_buffs.get_mut(tag).unwrap();
        self.m_client_buffs_mutex.unlock();
        loop {
            if client.current_len > 0 && client.ring_buff.getStoredDataSize() >= client.current_len {
                let pop_ret = client.ring_buff.pop(client.current_len);
                match pop_ret {
                    Ok(data) => {
                        process_func(client.current_id,data);
                    },
                    Err(_)=> {
                        log!("error read data");
                        break;
                    }
                }
                client.current_len = 0;
                client.current_id = 0;
            } else if client.current_len == 0 && client.ring_buff.getStoredDataSize() >= std::mem::size_of::<u32>(){
                client.current_id = client.ring_buff.getU32().unwrap();
                client.current_len = message_parser::MessageParser::get_id_size(client.current_id);
            } else {
                break;
            }
        }

    }
}