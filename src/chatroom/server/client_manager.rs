use std::hash::Hash;
use std::sync::Arc;
use std::collections::HashMap;
use crate::chatroom::message::chat_message::ChatMessage;
use crate::{concurrent::TarMutex, net::TarSocketOutputStream};
use crate::net::TarSocket;
use crate::chatroom::message::message_id;
use crate::tools::stringhelper;
use crate::chatroom::message::file_transfer_message;
use crate::log;
pub struct ClientInfo {
    name:String,
    socket:Arc<TarSocket>
}

impl ClientInfo {
    pub fn new(name:String,socket:Arc<TarSocket>)->Self {
        ClientInfo {
            name:name,
            socket:socket
        }
    }

    pub fn get_name(&self)->String {
        return self.name.clone();
    }

    pub fn get_socket(&self)->Arc<TarSocket> {
        return self.socket.clone();
    }
}

pub struct ClientManager {
    clients:HashMap<String,ClientInfo>, //<name,clientInfo>
    tag_name_maps:HashMap<String,String>,//<ip+port,name>
    clients_mutex:TarMutex
}

impl ClientManager {
    pub fn new()->Self {
        ClientManager {
            clients:HashMap::new(),
            clients_mutex:TarMutex::new(),
            tag_name_maps:HashMap::new()
        }
    }

    pub fn add_new_client(&mut self,name:&String,old_name:&String,socket:Arc<TarSocket>)->u32 {
        let client = ClientInfo::new(name.clone(), socket);
        let tag = client.socket.get_addr_string();
        log!("add_new_client trace1,name is {},tag is {}",name,tag);
        self.clients_mutex.lock();
        if !old_name.is_empty() {
            log!("add_new_client trace2");
            if self.clients.contains_key(old_name) || !self.clients.contains_key(name) {
                self.clients.remove(&tag);
                self.tag_name_maps.remove(name);
            } else {
                return message_id::Result_Already_Exist;
            }
        } else {
            log!("add_new_client trace3");
            let info = self.clients.get(name);
            match &info {
                Some(client_info)=> {
                    log!("add_new_client trace4");
                    if client_info.name.eq(name) {
                        self.clients_mutex.unlock();
                        return message_id::Result_Already_Exist;
                    }
                },
                None => {
                    
                }
            }
        }
        log!("add_new_client trace5");
        self.clients.insert(name.clone(), client);
        self.tag_name_maps.insert(tag, name.clone());
        self.clients_mutex.unlock();
        log!("add_new_client trace6");
        message_id::Result_Ok
    }

    pub fn remove_client(&mut self,tag:&String) {
        self.clients_mutex.lock();
        let name_result = self.tag_name_maps.get(tag);
        if let Some(name) = name_result {
            self.clients.remove(name);
        }
        
        self.tag_name_maps.remove(tag);
        self.clients_mutex.unlock();
    }

    pub fn send_msg(&self,msg:ChatMessage) {
        self.clients_mutex.lock();
        let client = self.clients.get(&msg.get_msg_to());
        self.clients_mutex.unlock();

        if let Some(c) = client {
            c.socket.get_output_stream().write(msg.to_bytes().as_slice());
        }
    }

    pub fn send_file_msg(&self,msg:file_transfer_message::FileTransferMessage) {
        self.clients_mutex.lock();
        let client = self.clients.get(&msg.get_to());
        self.clients_mutex.unlock();

        if let Some(c) = client {
            c.socket.get_output_stream().write(msg.to_bytes().as_slice());
        }
    }

    pub fn get_all_users(&self)->Vec<String> {
        let mut result = Vec::<String>::new();
        self.clients_mutex.lock();
        for entry in &self.clients {
            result.push(entry.0.clone());
        }
        self.clients_mutex.unlock();
        return result;
    }

    pub fn get_all_users_without_someone(&self,name:&String)->Vec<String> {
        let mut result = Vec::<String>::new();
        self.clients_mutex.lock();
        for entry in &self.clients {
            if !entry.0.eq(name) {
                result.push(entry.0.clone());
            }
        }
        self.clients_mutex.unlock();
        return result;
    }

    pub fn userinfo_foreach(&self,func:impl Fn(&ClientInfo)) {
        self.clients_mutex.lock();
        for entry in &self.clients {
            func(entry.1);
        }
        self.clients_mutex.unlock();
    }

    pub fn user_tag_to_name(&self,tag:&String)->String {
        self.clients_mutex.lock();
        let name_ret = self.tag_name_maps.get(tag);
        if let Some(name) = name_ret {
            self.clients_mutex.unlock();
            return name.clone();
        }
        self.clients_mutex.unlock();
        return String::from("");
    }
}