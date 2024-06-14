use std::{collections::HashMap, fs::create_dir_all, hash::Hash, sync::Mutex};
use serde::{Deserialize, Serialize};
use crate::io::{TarFile, TarIoStream, TarFileOutputStream,TarInputFileStream};
use crate::chatroom::message::{message_id,file_transfer_message};
use crate::log;

const PROFILE_DIR:&str = "./";

pub const LOCATION_IDLE:u32 = 1;
pub const LOCATION_GROUP:u32 = 2;
pub const LOCATION_CHATROOM:u32 = 3;

pub struct LocationManager {
    current_group_room:Mutex<(String,String)> //(group,room)
}

impl LocationManager {
    pub fn new()->Self {
        LocationManager {
            current_group_room:Mutex::new((String::from(""),String::from("")))
        }
    }

    pub fn current(&self)->u32 {
        let lock = self.current_group_room.lock().unwrap();
        if lock.0 != "" {
            return LOCATION_GROUP;
        } else if lock.1 != "" {
            return LOCATION_CHATROOM;
        }

        LOCATION_IDLE
    }

    pub fn enter_group(&self,group:&String) {
        let mut lock = self.current_group_room.lock().unwrap();
        lock.0 = group.clone();
    }

    pub fn enter_room(&self,room:&String) {
        let mut lock = self.current_group_room.lock().unwrap();
        lock.1 = room.clone();
    }

    pub fn get_group_name(&self)->String {
        let lock = self.current_group_room.lock().unwrap();
        lock.0.clone()
    }

    pub fn get_room_name(&self)->String {
        let lock = self.current_group_room.lock().unwrap();
        lock.1.clone()
    }

    pub fn leave_group(&self) {
        let mut lock = self.current_group_room.lock().unwrap();
        lock.0 = String::from("");
    }

    pub fn leave_room(&self) {
        let mut lock = self.current_group_room.lock().unwrap();
        lock.1 = String::from("");
    }
}

pub struct GroupMember {
    name:String,
    online:u32 //0:offline,1:online
}

pub struct Group {
    owner:String,
    groups_config:HashMap<String,Vec<String>>,
}

impl Group {
    pub fn new()->Self {
        Group {
            owner:String::from(""),
            groups_config:HashMap::new(),
        }
    }
    pub fn import(&mut self,path:&String) {
        let file = TarFile::new(path.to_string() + "/" + &self.owner + "/group.json");
        if !file.exists() {
            return;
        }
        let mut stream = TarInputFileStream::new(&file);
        let data = stream.read_all();
        self.groups_config = serde_json::from_str(&String::from_utf8(data).unwrap()).unwrap();
    }

    pub fn export(&self,path:String) {
        let json = serde_json::to_string(&self.groups_config).unwrap();
        let file = TarFile::new(path + "/" + &self.owner + "/group.json");
        file.create_new_file();
        let mut stream = TarFileOutputStream::new_truncate_stream(&file);
        stream.write(json.as_bytes());
    }

    pub fn add_new_group(&mut self,group_name:&String) {
        self.groups_config.insert(group_name.clone(), Vec::new());
    }

    pub fn add_new_users_to_group(&mut self,users:&Vec<String>,group_name:&String) {
        let list1 = self.groups_config.get_mut(group_name).unwrap();
        for user in users {
            list1.push(user.clone());
        }
    }

    pub fn remove_users_from_group(&mut self,users:&Vec<String>,group_name:&String) {
        let list1 = self.groups_config.get_mut(group_name).unwrap();
        for user in users {
            list1.retain(|name| !name.eq(user));
        }
    }

    pub fn erase_group(&mut self,group_name:&String) {
        self.groups_config.remove(group_name);
    }

    pub fn get_group_list(&mut self)->Vec<String> {
        let mut result:Vec<String> = Vec::new();
        for(name,_) in &self.groups_config {
            result.push(name.clone());
        }
        return result;
    }

    pub fn get_user_list(&self,group_name:&String)->Vec<String> {
        let mut result:Vec<String> = Vec::new();
        let list = self.groups_config.get(group_name).unwrap();

        for name in list {
            result.push(name.clone());
        }
        return result;
    }

    pub fn dump(&self) {
        for (name,list) in &self.groups_config {
            print!("Group:{}",name);
            for user in list {
                print!(" {} ",user);
            }
            log!("{}","\r\n");
        }
    }
}

pub struct ClientName {
    name:String,
    apply_new_name:String,
}
pub struct ClientManager {
    name:Mutex<ClientName>,
    chatters:Mutex<Vec<String>>,
    pub group:Mutex<Group>,
    pub location_mgr:LocationManager,
    //current_group_room:Mutex<(String,String)> //(group,room)
}

impl ClientManager {
    pub fn new()->Self {
        ClientManager {
            name:Mutex::new(ClientName{
                name:String::from(""),
                apply_new_name:String::from("")
            }),
            chatters:Mutex::new(Vec::<String>::new()),
            group:Mutex::new(Group::new()),
            location_mgr:LocationManager::new()
        }
    }

    pub fn back_from_group(&self) {
        self.location_mgr.leave_group();
    }

    pub fn back_from_room(&self) {
        self.location_mgr.leave_room();
    }

    pub fn move_to_group(&self,group_name:&String) {
        self.location_mgr.enter_group(group_name);
    }

    pub fn move_to_chat_room(&self,room_name:&String) {
        self.location_mgr.enter_room(room_name);
    }
    pub fn set_name(&self,name:&String) {
        {
            let mut lock = self.name.lock().unwrap();
            lock.name = name.clone();
        }

        //create profiler
        create_dir_all(PROFILE_DIR.to_string() + "/" + name);

        {
            let mut lock = self.group.lock().unwrap();
            lock.import(&PROFILE_DIR.to_string());
        }
    }

    pub fn set_apply_name(&self,name:&String) {
        let mut lock = self.name.lock().unwrap();
        lock.apply_new_name = name.clone();
    }

    pub fn get_name(&self)->String {
        let mut lock = self.name.lock().unwrap();
        return lock.name.clone();
    }

    pub fn get_apply_name(&self)->String {
        let mut lock = self.name.lock().unwrap();
        return lock.apply_new_name.clone();
    }

    pub fn add_one_chatter(&self,name:&String) {
        let mut lock = self.chatters.lock().unwrap();
        lock.push(name.clone());
    }

    pub fn set_chatters(&self,names:Vec<String>) {
        let mut lock = self.chatters.lock().unwrap();
        lock.clear();
        lock.extend(names);
    }

    pub fn get_chatters(&self)->Vec<String> {
        let mut result = Vec::<String>::new();

        //we should check wether we are in 
        let location = self.location_mgr.current();
        match location {
            LOCATION_IDLE=>{
                //TODO
            },
            LOCATION_GROUP=> {
                let group_name = self.location_mgr.get_group_name();
                let group = self.group.lock().unwrap();
                return group.get_user_list(&group_name);
            },
            LOCATION_CHATROOM=> {
                //TODO
            },
            _ => {
                //TODO
            }
        }
        
        let lock = self.chatters.lock().unwrap();
        for user in lock.iter() {
            result.push(user.clone());
        }
        return result;

    }
    pub fn remove_one_chatter(&self,name:&String) {
        let mut lock = self.chatters.lock().unwrap();
        lock.retain(|user_name| name != user_name);
    }
}

//pending task manager
pub const PendingTaskWaitingConfirm:u32 = 1;
pub const PendingTaskTransfering:u32 = 2;
pub const PendingTaskUnKnown:u32 = 100;

struct PendingCounter {
    count:Mutex<u32>
}

impl PendingCounter {
    fn new()->Self {
        PendingCounter {
            count:Mutex::new(1)
        }
    }

    fn get(&self)->u32 {
        let mut count = self.count.lock().unwrap();
        let ret = *count;
        *count += 1;
        return ret;
    }
}

pub struct PendingTask {
    message_id:u32,
    pending_status:u32,
    pending_id:u32,
    process:u32,

    //file transfer info
    from:String,
    to:String,
    file_name:String,
    source_ip:String,
    port:u32,
    file_size:u32,
}

impl PendingTask {
    pub fn new(pending_id:u32)->Self {
        PendingTask {
            message_id:message_id::Id_Message_Undefined,
            pending_status:PendingTaskUnKnown,
            from:String::from(""),
            to:String::from(""),
            file_name:String::from(""),
            source_ip:String::from(""),
            file_size:0,
            port:0,
            pending_id:pending_id,
            process:0,
        }
    }

    pub fn get_message_id(&self)->u32 {
        return self.message_id;
    }
    pub fn get_pending_id(&self)->u32 {
        return self.pending_id;
    }

    pub fn get_from(&self)->String {
        self.from.clone()
    }

    pub fn get_to(&self)->String {
        self.to.clone()
    }

    pub fn get_file_name(&self)->String {
        self.file_name.clone()
    }

    pub fn get_source_ip(&self)->String {
        self.source_ip.clone()
    }

    pub fn get_port(&self)->u32 {
        self.port
    }

    pub fn get_file_size(&self)->u32 {
        self.file_size
    }

    pub fn update_status(&mut self,pending_status:u32) {
        self.pending_status = pending_status;
    }

    pub fn get_status(&self)->u32 {
        self.pending_status
    }

    pub fn update_process(&mut self,process:u32) {
        self.process = process;
    }

    pub fn get_process(&self)->u32 {
        self.process
    }
}

pub struct PendingTaskManager {
    pending_tasks:Mutex<Vec<PendingTask>>,
    pending_id_counter:PendingCounter,
}

impl PendingTaskManager {
    pub fn new()->Self {
        PendingTaskManager {
            pending_tasks:Mutex::new(Vec::new()),
            pending_id_counter:PendingCounter::new()
        }
    }

    pub fn create_file_transfer_pending_task(&self,message:file_transfer_message::FileTransferMessage)->PendingTask {
        PendingTask {
            message_id:message_id::Id_FileTransfer,
            pending_status:PendingTaskWaitingConfirm,
            from:message.get_from(),
            to:message.get_to(),
            file_name:message.get_name(),
            source_ip:message.get_ip(),
            port:message.get_port(),
            file_size:message.get_file_size(),
            pending_id:self.pending_id_counter.get(),
            process:0,
        }
    }

    pub fn find_pending_task_info(&self,id:u32)->(String,u32,String,u32) {
        let mut lock = self.pending_tasks.lock().unwrap();
        for task in &*lock {
            if task.pending_id == id {
                return (task.get_source_ip(),task.get_port(),task.get_file_name(),task.get_file_size());
            }
        }

        return(String::from(""),0,String::from(""),0);
    }

    pub fn update_pending_task_status(&self,id:u32,status:u32) {
        let mut lock: std::sync::MutexGuard<Vec<PendingTask>> = self.pending_tasks.lock().unwrap();
        let mut hit = false;
        let mut index:usize = 0;

        for mut task in &*lock {
            if task.pending_id == id {
                hit = true;
                break;
            }
            index += 1;
        }

        if hit {
            lock.get_mut(index).unwrap().update_status(status);
        }
    }
    pub fn add_task(&self,task:PendingTask) {
        let mut lock = self.pending_tasks.lock().unwrap();
        lock.push(task);
    }

    pub fn remove(&self,pending_id:u32) {
        let mut lock = self.pending_tasks.lock().unwrap();
        lock.retain(|task| task.pending_id != pending_id);
    }


    pub fn foreach(&self,func:impl Fn(&PendingTask)) {
        let mut lock = self.pending_tasks.lock().unwrap();
        for task in &*lock {
            func(task);
        }
    }
    
}