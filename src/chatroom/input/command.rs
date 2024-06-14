use crate::io::TarFile;
use crate::chatroom::message;
use crate::log;

pub const Command_Connect:&str = "connect";

//----------------- ConnectCommand ---------------
pub struct ConnectCommand {
    ip:String,
    port:u32,
    name:String
}

impl ConnectCommand {
    pub fn print_standard_command() {
        log!("connect ip=xxx port=xxx name=xxx");
    }

    pub fn from_string(content:&str)->Option<ConnectCommand> {
        let mut iter = content.split(",");
        let mut command = ConnectCommand {
            ip:String::from(""),
            port:0,
            name:String::from("")
        };

        let mut count = 0;

        for item in iter {
            let cmd = item.trim();

            if cmd.starts_with("ip") {
                let mut iter = cmd.split("=");
                iter.next();
                let ip = iter.next().unwrap();
                command.ip = ip.to_string();
                count += 1;
            } else if cmd.starts_with("port") {
                let mut iter = cmd.split("=");
                iter.next();
                let port = iter.next().unwrap();
                command.port = port.parse().unwrap();
                count += 1;
            } else if cmd.starts_with("name") {
                let mut iter = cmd.split("=");
                iter.next();
                let name = iter.next().unwrap();
                command.name = name.to_string();
                count += 1;
            }
        }
        
        if count == 3 {
            return Some(command)
        }

        None
    }

    pub fn get_ip(&self)->String {
        return self.ip.clone();
    }

    pub fn get_port(&self)->u32 {
        return self.port;
    }

    pub fn get_name(&self)->String {
        return self.name.clone();
    }
}

//----------------- MessageCommand ---------------
pub struct MessageCommand {
    name:String,
    text:String
}

impl MessageCommand {
    pub fn print_standard_command() {
        log!("msg name(e.g wang),text content");
    }

    pub fn from_string(content:&str)->Option<MessageCommand> {
        let first_split: Option<usize> = content.find(",");
        match first_split {
            Some(index)=> {
                let name = content[0..index].to_string();
                let text = content[index+1..].to_string();
                return Some(MessageCommand {
                    name:name,
                    text:text
                });
            },
            None=> {
            }
        }
        None
    }

    pub fn get_name(&self)->String {
        return self.name.clone();
    }

    pub fn get_text(&self)->String {
        return self.text.clone();
    }
}

pub struct NameCommand {
    name:String
}

impl NameCommand {
    pub fn print_standard_command() {
        log!("name xxxx(e.g wang)");
    }

    pub fn from_string(content:&str)->Option<NameCommand> {
        Some(NameCommand {
            name:content.to_string()
        })
    }

    pub fn get_name(&self)->String {
        return self.name.clone();
    }
}

//list
pub const LIST_CHATTERS_TAG:u32 = 1;
pub const LIST_TASK_TAG:u32 = 2;

pub struct ListCommand {
    tag:u32
}

impl ListCommand {
    pub fn print_standard_command() {
        log!("list xxxx(e.g chatter|task)");
        log!("if you only want to show users,type list directly");
    }

    pub fn from_string(content:&str)->Option<ListCommand> {
        if content == "list" || content == "" || content.len() == 0 {
            return Some(ListCommand {
                tag:LIST_CHATTERS_TAG
            });
        } else if content == "task" {
            return Some(ListCommand {
                tag:LIST_TASK_TAG
            });
        }
        
        None
    }

    pub fn get_tag(&self)->u32 {
        return self.tag;
    }
}


//file
pub struct FileCommand {
    to:String,
    path:String
}

impl FileCommand {
    pub fn print_standard_command() {
        log!("file wang,./test.txt");
    }

    pub fn from_string(content:&str)->Option<FileCommand> {
        let split_items:Vec<&str> = content.split(",").collect();
        if split_items.len() == 2 {
            let to = split_items[0];
            let path = split_items[1];

            return Some(FileCommand {
                to:to.to_string(),
                path:path.to_string()
            });
        }

        None
    }

    pub fn get_to_name(&self)->String {
        self.to.clone()
    }

    pub fn get_path(&self)->String {
        self.path.to_string()
    }
}

//Exit Command
pub struct ExitCommand {
}

impl ExitCommand {
    pub fn print_standard_command() {
        log!("type exit directly");
    }

}

//Broadcast command
pub struct BroadCastCommand {
    content:String,
}

impl BroadCastCommand {
    pub fn print_standard_command() {
        log!("broadcast hello,every one");
    }

    pub fn from_string(content:&str)->Option<BroadCastCommand> {
        Some(BroadCastCommand {
            content:content.to_string()
        })
    }

    pub fn get_content(&self)->String {
        self.content.clone()
    }
}

//Group command
pub const GroupAddUser:u32 = 1;
pub const GroupCreate:u32 = 2;
pub const GroupRemoveUser:u32 = 3;
pub const GroupErase:u32 = 4;
pub const GroupList:u32 = 5;
pub const GroupUnknow:u32 = 100;

pub struct GroupCommand {
    action:u32,
    group_name:String,
    user_names:Vec<String>,
}

enum GroupCommandStatus {
    Idle,
    New,
    Erase,
    GroupName,
    UserName,
}

impl GroupCommand {
    pub fn new()->Self {
        GroupCommand {
            action:GroupUnknow,
            group_name:String::from(""),
            user_names:Vec::new()
        }
    }

    pub fn print_standard_command() {
        log!("create a group:group new playgame");
        log!("erase a group:group erase work");
        log!("show all groups:group list");

        log!("add a user to a group:group xxxxx(group name),add xxxx,xxx,xxx(user name)");
        log!("remove a user to a group:group xxxxx(group name),remove xxxx,xxx,xxx(user name)");
    }

    pub fn from_string(content:&str)->Option<GroupCommand> {
        let mut command = GroupCommand::new();
        //find first command
        let mut parse_content = content.trim();
        let result = parse_content.find(" ");
        match result {
            Some(index)=> {
                let action = &parse_content[0..index];
                if action.eq("new") {
                    Self::parse_new_action(&parse_content[index + 1..parse_content.len()],&mut command);
                    return Some(command);
                } else if action.eq("erase") {
                    Self::parse_erase_action(    &parse_content[index + 1..parse_content.len()],&mut command);
                    return Some(command);
                } else {
                    //this action is add/remove
                    //group xxxxx(group name),add xxxx,xxx,xxx(user name)
                    let split_str = parse_content.find(",");
                    match split_str {
                        Some(index) => {
                            let group_name = &parse_content[0..index];
                            command.group_name = group_name.to_string().trim().to_string();
                            Self::parse_group_action(&parse_content[index + 1..parse_content.len()], &mut command);
                            if command.action != GroupUnknow { 
                                return Some(command);
                            } else {
                                return None;
                            }
                        },
                        None => {
                            return None;
                        }
                    }
                }
            },
            None => {
                //parse list
                if content.contains("list") {
                    command.action = GroupList;
                    return Some(command);
                }
            }
        }
        None
    }

    pub fn parse_new_action(content:&str,command:&mut GroupCommand) {
        let group_name = content.trim();
        command.group_name = group_name.to_string();
        command.action = GroupCreate;
    }

    pub fn parse_erase_action(content:&str,command:&mut GroupCommand) {
        let group_name = content.trim();
        command.group_name = group_name.to_string();
        command.action = GroupErase;
    }

    pub fn parse_group_action(content:&str,command:&mut GroupCommand) {
        let parse_content = content.trim();
        let result = parse_content.find(" ");
        match result {
            Some(index) => {
                let command_str = &parse_content[0..index];
                if command_str == "add" {
                    command.action = GroupAddUser;
                } else if command_str == "remove" {
                    command.action = GroupRemoveUser;
                } else {
                    return;
                }

                //start parse user
                Self::parse_group_user(&parse_content[index+1..parse_content.len()], command);
            },
            None => {
                //Do Nothing
            }
        }
    }

    pub fn parse_group_user(content:&str,command:&mut GroupCommand) {
        let parse_content = content.trim();
        let split_str = parse_content.split(",");
        for user in split_str {
            let user_string = user.trim().to_string();
            if user_string.len() > 0 {
                command.user_names.push(user_string);
            }
        }
    }

    pub fn get_action(&self)->u32 {
        self.action
    }

    pub fn get_group_name(&self)->String {
        self.group_name.clone()
    }

    pub fn get_user_names(&self)->&Vec<String> {
        &self.user_names
    }

}

//enter
pub struct EnterCommand {
    group_name:String,
    chat_room_name:String,
}

impl EnterCommand {
    pub fn print_standard_command() {
        log!("enter group/room xxxxx");
    }

    pub fn from_string(content:&str)->Option<EnterCommand> {
        let parse_content = content.trim();
        let result = content.find(" ");
        match result {
            Some(index) => {
                let tag = &parse_content[0..index];
                if tag.eq("group") {
                    let group_name = parse_content[index + 1..parse_content.len()].trim();
                    return Some(EnterCommand {
                        group_name:group_name.to_string(),
                        chat_room_name:String::from("")
                    });
                } else if tag.eq("room") {
                    let room_name = parse_content[index + 1..parse_content.len()].trim();
                    return Some(EnterCommand {
                        group_name:String::from(""),
                        chat_room_name:room_name.to_string()
                    });
                }
            },
            None => {}
        }
        None
    }

    pub fn get_group_name(&self)->String {
        self.group_name.clone()
    }

    pub fn get_chat_room_name(&self)->String {
        self.chat_room_name.clone()
    }
}

//back command
struct BackCommand {
}

impl BackCommand {
    
}

//Task Command
pub const Task_List:u32 = 1; 
pub const Task_Stop:u32 = 2;
pub const Task_Confirm:u32 = 3;

pub struct TaskCommand {
    action:u32,
    pending_task_id:u32,
    confirm_result:bool
}

impl TaskCommand {
    pub fn print_standard_command() {
        log!("list pending task:task list");
        log!("confirm one task:task xx(id) confirm xx(Y/N)");
        log!("stop one task:task xx(id) stop");
    }

    pub fn from_string(content:&str)->Option<TaskCommand> {
        let mut command = TaskCommand {
            action:0,
            pending_task_id:0,
            confirm_result:false
        };

        //find first command
        let mut parse_content = content.trim();
        let result = parse_content.find(" ");
        match result {
            Some(index) => {
                //first is id
                let id_str = &parse_content[0..index];
                log!("str is {}",id_str);
                let pending_id:u32 = id_str.parse().unwrap();
                let rest_command = parse_content[index+1..].trim();
                if rest_command.eq("stop") {
                    command.action = Task_Stop;
                    command.pending_task_id = pending_id;
                    return Some(command)
                }

                let confirm_ret = rest_command.find(" ");
                if let Some(index) = confirm_ret {
                    if rest_command[0..index].eq("confirm") {
                        let confirm_result = rest_command[index+1..].trim();
                        if confirm_result.eq("Y") {
                            command.action = Task_Confirm;
                            command.pending_task_id = pending_id;
                            command.confirm_result = true;
                            return Some(command);
                        } else if confirm_result.eq("N") {
                            command.action = Task_Confirm;
                            command.pending_task_id = pending_id;
                            command.confirm_result = false;
                            return Some(command);
                        }
                    }
                }
            },
            None => {
                //parse list
                if content.contains("list") {
                    command.action = Task_List;
                    return Some(command);
                }
            }
        }
        None
    }

    pub fn get_action(&self)->u32 {
        self.action
    }

    pub fn get_pending_task_id(&self)->u32 {
        self.pending_task_id
    }

    pub fn get_confirm_result(&self)->bool {
        self.confirm_result
    }

}

//Room Command
pub struct RoomCommand {
    event:u32,
    room_name:String,
    msg:String
}

impl RoomCommand {
    pub fn print_standard_command() {
        log!("create a chat room:rom new playgame");
        log!("erase a chat room:rom erase work");
        log!("show all room:room list");
    }
    
    pub fn from_string(content:&str)->Option<RoomCommand> {
        let mut command = RoomCommand {
            event:0,
            room_name:String::from(""),
            msg:String::from("")
        };

        //find first command
        let mut parse_content = content.trim();
        
        if parse_content.eq("list") {
            command.event = message::room_message::RoomEventList;
            return Some(command);
        }

        let result = parse_content.find(" ");
        if let Some(action_index) = result {
            let action = &parse_content[0..action_index];
            if action.eq("new") {
                command.event = message::room_message::RoomEventCreate;
            } else if action.eq("erase") {
                command.event = message::room_message::RoomEventErase;
            } else {
                return None;
            }


            command.room_name = parse_content[action_index+1..].trim().to_string();
            return Some(command);
        }

        None
    }

    pub fn get_event(&self)->u32 {
        self.event
    }

    pub fn get_room_name(&self)->String {
        return self.room_name.clone();
    }

    pub fn get_message(&self)->String {
        return self.msg.clone();
    }
}

//history command
pub struct HistoryCommand {
    user:String,
}

impl HistoryCommand {
    pub fn print_standard_command() {
        log!("show chat history:history xxxx(username)");
    }
    
    pub fn from_string(content:&str)->Option<HistoryCommand> {
        let mut parse_content = content.trim();
        if parse_content.len() > 0 {
            return Some(HistoryCommand {
                user:parse_content.to_string()
            });
        }

        None
    }

    pub fn get_name(&self)->String {
        self.user.clone()
    }
}