use std::collections::HashMap;
use std::sync::Mutex;

use crate::chatroom::message::message_id;

pub struct RoomInfo {
    users:Vec<String>,
    owner:String
}

pub struct RoomManager {
    rooms:Mutex<HashMap<String,RoomInfo>>
}

impl RoomManager {
    pub fn new()->Self {
        RoomManager {
            rooms:Mutex::new(HashMap::new())
        }
    }

    pub fn create_room(&self,name:&String,owner:&String)->u32 {
        let mut lock = self.rooms.lock().unwrap();
        if lock.contains_key(name) {
            return message_id::Result_Already_Exist;
        }

        lock.insert(name.clone(), RoomInfo{
            users:Vec::new(),
            owner:owner.clone()
        });
        return message_id::Result_Ok;
    }

    pub fn on_user_enter_room(&self,room_name:&String,user_name:&String) {
        let mut lock = self.rooms.lock().unwrap();
        let result = lock.get_mut(room_name);
        if let Some(room_info) = result {
            room_info.users.push(user_name.to_string());
        }
    }

    pub fn on_user_leave_room(&self,room_name:&String,user_name:&String) {
        let mut lock = self.rooms.lock().unwrap();
        let result = lock.get_mut(room_name);
        if let Some(room_info) = result {
            room_info.users.retain(|user| !user.eq(user_name));
        }
    }
    
    pub fn userinfo_foreach(&self,room:&String,func:impl Fn(&String)) {
        let mut lock = self.rooms.lock().unwrap();
        let room_info_ret = lock.get(room);

        if let Some(room_info) = room_info_ret {
            for user in &room_info.users {
                func(&user);
            }
        }
    }

    pub fn get_users(&self,room_name:&String)->String {
        let mut users = String::from("");
        let mut lock = self.rooms.lock().unwrap();
        let room_info_ret = lock.get(room_name);

        if let Some(room_info) = room_info_ret {
            for user in &room_info.users {
                users += ",";
                users += user;
            }   
        }

        return users;
    }

    pub fn get_rooms(&self)->String {
        let mut rooms = String::from("");
        let mut lock = self.rooms.lock().unwrap();
        for (name,_) in &*lock {
            rooms += name;
            rooms += ","
        }

        return rooms;
    }
}