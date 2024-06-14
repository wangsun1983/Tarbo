use std::io;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use futures::future::Pending;
use libc::exit;


use crate::chatroom::input;
use crate::chatroom::input::command::GroupCommand;
use crate::chatroom::input::command::RoomCommand;
use crate::chatroom::input::parser::parse_message_command;
use crate::chatroom::message::broadcast_message;
use crate::chatroom::message::chat_message::{self, ChatMessage};
use crate::chatroom::message::file_transfer_message;
use crate::chatroom::message::file_transfer_message::FileTransferMessage;
use crate::chatroom::message::room_message;
use crate::chatroom::message::room_message::RoomMessage;
use crate::chatroom::server::client_manager;
use crate::coroutine::Filament;
use crate::io::{TarFile, TarFileOutputStream,TarInputFileStream, TarIoStream};
use crate::log;
use crate::net::{TarSocketMonitor,TarSocket,TarSocketBuilder,
                TarAddress,TarListener,Event};
use crate::lang::TarByteRingArray;
use crate::concurrent::{TarBlockingQueue, TarExecutor, TarThreadPoolExecutor};

use crate::chatroom::message::{self, chatter_list_message, common_ack_message, new_comer_message, user_exit_message};
use crate::chatroom::input::{command, parser};
use crate::chatroom::message::message_id;
use crate::chatroom::message::message_parser;
use super::manager::{PendingTask,PendingTaskWaitingConfirm,PendingTaskTransfering};
use super::manager::{ClientManager,PendingTaskManager};
use super::record_center::RecordCenter;
use crate::chatroom::client::{manager};
use crate::chatroom::utils::display;
use crate::chatroom::client::record_center;

struct PortCounter {
    port:Mutex<u32>
}

impl PortCounter {
    fn new()->Self {
        PortCounter {
            port:Mutex::new(1000)
        }
    }

    fn get(&self)->u32 {
        let mut count = self.port.lock().unwrap();
        let ret = *count;
        *count += 1;
        return ret;
    }
}


struct Task {
    id:u32,
    data:Vec<u8>
}

struct ChatClientListener {
    ring_buff:TarByteRingArray,
    current_len:usize,
    current_id:u32,
    task_pool:Arc<TarBlockingQueue<Task>>,
    executor:TarThreadPoolExecutor,
    manager:Arc<ClientManager>,
    port_counter:Arc<PortCounter>,
    pending_manager:Arc<PendingTaskManager>,
    record_center:Arc<RecordCenter>,
}

impl ChatClientListener {
    pub fn new(manager:Arc<ClientManager>,
                port_counter:Arc<PortCounter>,
                pending_manager:Arc<PendingTaskManager>,
                record_center:Arc<RecordCenter>)->Self {
        let task_pool = Arc::new(TarBlockingQueue::new());
        
        let mut mgr = ChatClientListener {
            ring_buff:TarByteRingArray::new(),
            current_id:0,
            current_len:0,
            task_pool:task_pool.clone(),
            executor:TarThreadPoolExecutor::new(10),
            manager:manager.clone(),
            port_counter:port_counter.clone(),
            pending_manager:pending_manager.clone(),
            record_center:record_center.clone(),
        };

        let fila = Arc::new(Filament::new());

        for i in 0..10 {
            let task_c: Arc<TarBlockingQueue<Task>> = task_pool.clone();
            let client_manager = manager.clone();
            let fila_c = fila.clone();
            let pending_mgr = pending_manager.clone();
            let record_center = record_center.clone();
            mgr.executor.submit(move || {
                loop {
                    let task = task_c.takeFirst();
                    log!("task.id is {}",task.id);

                    match task.id {
                        message_id::Id_BroadCast => {
                            let msg = broadcast_message::BroadCastMessage::from_bytes(&task.data);
                            display::display_one_broadcast(&msg.0.get_msg_from(),&msg.0.get_content());
                        },
                        message_id::Id_CommonAck => {
                            let msg: (common_ack_message::CommonAckMessage, usize) = common_ack_message::CommonAckMessage::from_bytes(&task.data);
                            match msg.0.get_result() {
                                message_id::Result_Ok => {
                                    if msg.0.get_id() == message_id::Id_NewComer {
                                        //update all config for client
                                        client_manager.set_name(&client_manager.get_apply_name());
                                        record_center.update_user_name(&client_manager.get_apply_name());
                                    }
                                },
                                message_id::Result_Already_Exist => {
                                    if msg.0.get_id() == message_id::Id_NewComer {
                                        display::display_error(&String::from("User Name already exists"));
                                        //TODO close local socket
                                        
                                    }
                                },
                                _ => {}
                            }
                        },
                        message_id::Id_NewComer => {
                            let new_comer_msg = new_comer_message::NewComerMessage::from_bytes(&task.data);
                            log!("client,new commer {}",new_comer_msg.0.get_name());
                            client_manager.add_one_chatter(&new_comer_msg.0.get_name());
                        },
                        message_id::Id_UserExit => {
                            let new_comer_msg: (user_exit_message::UserExitMessage, usize) = user_exit_message::UserExitMessage::from_bytes(&task.data);
                            client_manager.remove_one_chatter(&new_comer_msg.0.get_name());
                        },
                        message_id::Id_ChatMessage => {
                            let chatmsg = chat_message::ChatMessage::from_bytes(&task.data);
                            display::display_one_message(&chatmsg.0.get_msg_from(),&chatmsg.0.get_content());
                            //save to record
                            record_center.add_chat_record(&chatmsg.0.get_msg_from(), &chatmsg.0.get_msg_from(),&chatmsg.0.get_content())
                        },
                        message_id::Id_ChatterList => {
                            log!("client new comer!!!!");
                            let users = chatter_list_message::ChatterListMessage::from_bytes(&task.data);
                            log!("client new comer!!!!,users size is {}",users.len());
                            client_manager.set_chatters(users);
                        },
                        message_id::Id_FileTransfer => {
                            log!("client receive file transfer message trace1");
                            let file_transfer_message = file_transfer_message::FileTransferMessage::from_bytes(&task.data).0;
                            let pending_task = pending_mgr.create_file_transfer_pending_task(file_transfer_message);
                            display::display_pending_task(&pending_task, false);
                            pending_mgr.add_task(pending_task);
                        },
                        message_id::Id_RoomMessage => {
                            log!("client,accept room message trace1");
                            let room_message = room_message::RoomMessage::from_bytes(&task.data).0;
                            match room_message.get_event() {
                                room_message::RoomEventList => {
                                    let rooms_str = room_message.get_content();
                                    let rooms = rooms_str.split(",");
                                    let mut list = Vec::<String>::new();
                                    for room in rooms {
                                        if room.len() > 0 {
                                            list.push(room.to_string());
                                        }
                                    }

                                    display::display_list(&String::from("Room List"), &list);
                                },
                                room_message::RoomEventListUsers => {
                                    //TODO
                                },
                                room_message::RoomEventMsg => {
                                    log!("client,accept room message trace2");
                                    display::display_room_message(&room_message.get_who(), 
                                                                  &room_message.get_room_name(),
                                                               &room_message.get_content());
                                },
                                _=>{
                                    //TODO
                                }
                            }
                        }
                        _ => {
                            //TODO
                        }
                    }
                }
            });
        }

        mgr
    }
}

impl TarListener for ChatClientListener {
    fn on_event(&mut self,event:Event,data:Option<Vec<u8>>,sock:Arc<TarSocket>) {
        match event {
            Event::Connect=>{
                //TODO
            },
            Event::Disconnect=> {
                //TODO
            },
            Event::Message => {
                self.ring_buff.push(data.unwrap().as_slice());
                loop {
                    log!("on_event trace1 current_len is {},data size is {}",self.current_len,self.ring_buff.getStoredDataSize());
                    if self.current_len > 0 && 
                        self.ring_buff.getStoredDataSize() >= self.current_len {
                            let pop_ret = self.ring_buff.pop(self.current_len);
                            match pop_ret {
                                Ok(data) => {
                                    log!("on_event trace2");
                                    self.task_pool.putLast(Task{
                                        id:self.current_id,
                                        data:data
                                    });
                                },
                                Err(_)=> {
                                    log!("error read data");
                                    break;
                                }
                            }
                            self.current_len = 0;
                            self.current_id = 0;
                    } else if self.current_id == 0 && 
                        self.ring_buff.getStoredDataSize() >= std::mem::size_of::<u32>(){
                            self.current_id = self.ring_buff.getU32().unwrap();
                            self.current_len = message_parser::MessageParser::get_id_size(self.current_id);
                            log!("parse id is {},current_len is {}",self.current_id,self.current_len);
                    } else {
                        break;
                    }
                }

            }
            Event::NewClient => {
                //Do nothing
            }
        }
    }
}

const CLIENT_IDLE:u32 = 0;
const CLIENT_CONNECTED:u32 = 1;
const CLIENT_NAME_APPLY:u32 = 2;
const CLIENT_CHAT:u32 = 3;

pub struct ChatClient {
    monitor:TarSocketMonitor,
    port:u32,
    server_ip:String,
    socket:Arc<TarSocket>,
    status:u32,
    manager:Arc<ClientManager>,
    socket_builder:Arc<TarSocketBuilder>,
    port_counter:Arc<PortCounter>,
    pending_manager:Arc<PendingTaskManager>,
    record_center:Arc<RecordCenter>,
}

impl ChatClient {
    pub fn new()->Self {
        let mut client_socket = TarSocketBuilder::new().
                                create_socket(TarAddress::new("127.0.0.1",111));
        //client_socket.connect();
        let manager = Arc::new(ClientManager::new());
        let port_counter = Arc::new(PortCounter::new());
        let pending_manager: Arc<PendingTaskManager> = Arc::new(PendingTaskManager::new());
        let record_mgr = Arc::new(RecordCenter::new());

        ChatClient {
            monitor:TarSocketMonitor::new(),
            port:0,
            server_ip:String::from(""),
            socket:Arc::new(client_socket),
            status:CLIENT_IDLE,
            manager:manager.clone(),
            socket_builder:Arc::new(TarSocketBuilder::new()),
            port_counter:port_counter.clone(),
            pending_manager:pending_manager.clone(),
            record_center:record_mgr.clone()
        }
    }

    pub fn start(&mut self) {
        self.monitor.bind_as_client(
                self.socket.clone(), 
                Box::new(ChatClientListener::new(self.manager.clone(),
                                                            self.port_counter.clone(),
                                                        self.pending_manager.clone(),
                                                                        self.record_center.clone())));
    }

    pub fn send_new_comer_message(&self,name:&String) {
        let msg = new_comer_message::NewComerMessage::new(name.clone());
        self.socket.get_output_stream().write(msg.to_bytes().as_slice());
    }

    pub fn send_update_name_message(&self,name:&String) {
        let old_name = self.manager.get_name();
        let msg = new_comer_message::NewComerMessage::new_update_name(name.clone(), old_name.clone());
        self.socket.get_output_stream().write(msg.to_bytes().as_slice());
    }

    pub fn send_chat_message(&self,chat_msg:&ChatMessage) {
        self.socket.get_output_stream().write(chat_msg.to_bytes().as_slice());
    }

    pub fn process_command(&mut self) {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_)=> {
                    log!("input is {}",input);
                    let (cmd,content) = parser::parse_command(&input);
                    log!("client process command cmd {}",cmd);
                    match cmd {
                        message_id::LocalId_Client_History => {
                            let parse_ret = parser::parse_history_command(content);
                            if let Some(cmd) = parse_ret {
                                let result = self.record_center.get_chat_history(&cmd.get_name());
                                for (from,content,time) in result {
                                    display::display_hisotry_record(&from,&content,time);
                                }
                            }
                        },
                        message_id::Id_RoomMessage => {
                            let parse_ret = parser::parse_room_command(content);
                            if let Some(cmd) = parse_ret {
                                let msg = message::room_message::RoomMessage::new(cmd.get_event(),
                                                                                          &self.manager.get_name(),
                                                                                          &cmd.get_room_name(),
                                                                                          &cmd.get_message());
                                self.socket.get_output_stream().write(msg.to_bytes().as_slice());
                            }
                        },
                        message_id::LocalId_Client_Task => {
                            let cmd = parser::parse_task_command(content);
                            match cmd {
                                Some(command)=> {
                                    let action = command.get_action();
                                    if action == input::command::Task_Confirm {
                                        if command.get_confirm_result() {
                                            let fila_closure = self.socket_builder.get_fila().clone();
                                            let pending_manager_closure = self.pending_manager.clone();
                                            //update task status
                                            pending_manager_closure.update_pending_task_status(command.get_pending_task_id(), PendingTaskTransfering);
                                            self.socket_builder.get_fila().spawn(async move {
                                                let (ip,port,file_name,mut file_size) = pending_manager_closure.find_pending_task_info(command.get_pending_task_id());
                                                let mut file_output = TarFileOutputStream::new_truncate_stream(&TarFile::new(file_name));
                                                log!("client receive file transfer message trace2,ip is {},port is {}",ip,port);
                                                //start connect
                                                let mut socket = TarSocketBuilder::new_with_fila(fila_closure).create_socket(TarAddress::new(&ip,port));
                                                let client = socket.connect_async().await;
                                                log!("client receive file transfer message trace3");
                                                //log!("connect result is {}",client);
                                                let rx: Arc<crate::net::socketinputstream::TarSocketInputStream> = socket.get_input_stream();
                                                loop {
                                                    let data = rx.read_async().await.unwrap();
                                                    log!("client receive file transfer message trace4");
                                                    file_output.write(&data);
                                                    file_size -= data.len() as u32;
                                                    if file_size == 0 {
                                                        let tx = socket.get_output_stream();
                                                        let response:[u8;32] = [0;32];
                                                        tx.write_async(&response).await;
                                                        break;
                                                    }
                                                    log!("client receive file transfer message trace5");
                                                }
                                            });
                                        } else {
                                            //TODO
                                        }
                                    } else if action == input::command::Task_List {
                                        self.pending_manager.foreach(|task|{
                                            display::display_pending_task(task, true);
                                        })
                                    } else if action == input::command::Task_Stop {
                                        //TODO
                                    }
                                },
                                None =>{}
                            }
                        },
                        message_id::LocalId_Client_Back => {
                            //self.manager.back_from_group_chat_room();
                            let location = self.manager.location_mgr.current();
                            match location {
                                LOCATION_IDLE=>{
                                    //TODO
                                },
                                LOCATION_GROUP=> {
                                    self.manager.back_from_group();
                                },
                                LOCATION_CHATROOM=> {
                                    self.manager.back_from_room();
                                    let msg = RoomMessage::new(message::room_message::RoomEventLeave,
                                        &self.manager.get_name(),
                                        &self.manager.location_mgr.get_room_name(), 
                                        &String::from(""));
                                    self.socket.get_output_stream().write(msg.to_bytes().as_slice());
                                },
                                _ => {
                                    //TODO
                                }
                            }
                            //TODO
                        },
                        message_id::LocalId_Client_Enter=> {
                            let cmd = parser::parse_enter_command(content);
                            if let Some(command) = cmd {
                                let group_name = command.get_group_name();
                                let chat_room_name = command.get_chat_room_name();

                                if group_name.len() > 0 {
                                    self.manager.move_to_group(&group_name);
                                } else if chat_room_name.len() > 0 {
                                    self.manager.move_to_chat_room(&chat_room_name);
                                    //send message to server
                                    let msg = RoomMessage::new(message::room_message::RoomEventEnter,
                                                                            &self.manager.get_name(),
                                                                            &chat_room_name, 
                                                                            &String::from(""));
                                    self.socket.get_output_stream().write(msg.to_bytes().as_slice());
                                }
                            }
                        },
                        message_id::LocalId_Client_Group=> {
                            let comm = parser::parse_group_command(content);
                            if let Some(command) = comm {
                                let mut lock = self.manager.group.lock().unwrap();
                                match command.get_action() {
                                    command::GroupAddUser => {
                                        lock.add_new_users_to_group(&command.get_user_names(), &command.get_group_name());
                                    },
                                    command::GroupCreate => {
                                        lock.add_new_group(&command.get_group_name());
                                    },
                                    command::GroupErase => {
                                        lock.erase_group(&command.get_group_name());
                                    },
                                    command::GroupList => {
                                        let groups = lock.get_group_list();
                                        display::display_list(&String::from("GROUP"),&groups);
                                    },
                                    command::GroupRemoveUser => {
                                        lock.remove_users_from_group(command.get_user_names(), &command.get_group_name())
                                    },
                                    _ => {
                                        //do nothing
                                    }
                                }
                            }
                        },
                        message_id::Id_BroadCast=> {
                            let command_ret = parser::parse_broadcast_command(content);
                            if let Some(command) = command_ret {
                                let msg = broadcast_message::BroadCastMessage::new(&self.manager.get_name(), &command.get_content());
                                self.socket.get_output_stream().write(msg.to_bytes().as_slice());
                            }
                        },
                        message_id::Id_UserExit=> {
                            //there is no need to send exit info
                            break;
                        },
                        message_id::Id_NewComer=> {
                            if self.status == CLIENT_IDLE {
                                let command = parser::parse_connect_command(content).unwrap();
                                self.server_ip = command.get_ip();
                                self.port = command.get_port();
                                self.manager.set_apply_name(&command.get_name());
                                let mut client_socket = self.socket_builder.create_socket(TarAddress::new(&self.server_ip,self.port));
                                client_socket.connect();
                                
                                //if current connection exists,close it.
                                if self.socket.is_connected() {
                                    self.socket.close();
                                }
                                self.socket = Arc::new(client_socket);
                                self.start();
                                self.send_new_comer_message(&command.get_name())
                            } else {
                                let command = parser::parse_name_command(content).unwrap();
                                self.send_update_name_message(&command.get_name());
                                self.manager.set_apply_name(&command.get_name());
                                self.send_update_name_message(&command.get_name())
                            }
                        },
                        message_id::Id_ChatMessage=>{
                            let command = parser::parse_message_command(content).unwrap();
                            let send_msg = ChatMessage::new_msg(&self.manager.get_name(),&command.get_name(), &command.get_text());
                            self.send_chat_message(&send_msg);
                            log!("insert user to is {},who is {}",command.get_name(),self.manager.get_name());
                            self.record_center.add_chat_record(&command.get_name(),&self.manager.get_name(), &command.get_text());
                        },
                        message_id::LocalId_Client_List => {
                            let command = parser::parse_list_command(content).unwrap();
                            log!("local id client message,tag is {}",command.get_tag());
                            match command.get_tag() {
                                command::LIST_CHATTERS_TAG => {
                                    let users = self.manager.get_chatters();
                                    display::display_list(&String::from("USER"),&users);
                                },
                                command::LIST_TASK_TAG =>{
                                    //TODO
                                },
                                _=>{}
                            }
                        },
                        message_id::Id_FileTransfer => {
                            let command = parser::parse_file_command(content).unwrap();
                            let file = TarFile::new(command.get_path());
                            //1.get a usable port
                            let build_closure = self.socket_builder.clone();
                            let port_closure = self.port_counter.clone();
                            let client_sock_closure = self.socket.clone();
                            self.socket_builder.get_fila().spawn(async move{
                                loop {
                                    let port = port_closure.get();
                                    log!("client file server port is {}",port);
                                    let send_msg = 
                                        FileTransferMessage::new(&command.get_to_name(),&file.get_name().unwrap(),port,file.size().unwrap() as u32);

                                    let mut socket = build_closure.create_socket(TarAddress::new("127.0.0.1",port));
                                    let bind_ret = socket.bind_async().await;
                                    if bind_ret {
                                        client_sock_closure.get_output_stream().write_async(&send_msg.to_bytes()).await;
                                    } else {
                                        continue;
                                    }
                                    log!("client file server port trace1");
                                    let client: Option<TarSocket> = socket.accept_async().await;
                                    match client {
                                        Some(c)=> {
                                            //send file
                                            log!("client file server port trace2");
                                            let mut stream: TarInputFileStream = TarInputFileStream::new(&file);
                                            let mut buf:[u8;1024*32] = [0;1024*32];
                                            let sock_output = c.get_output_stream();
                                            loop {
                                                let read_ret = stream.read(&mut buf);
                                                log!("client file server port trace3");
                                                match read_ret {
                                                    Ok(size) => {
                                                        if size > 0 {
                                                            log!("client file server port trace4");
                                                            sock_output.write_async(&buf[0..size]).await;
                                                        } else {
                                                            //wait for client response  
                                                            let rx = c.get_input_stream();
                                                            rx.read_async().await;
                                                            break;
                                                        }
                                                    },
                                                    Error=> {
                                                        break;
                                                    }
                                                }                                            
                                            }
                                        },
                                        None=> {
                                            log!("no client", );
                                        }
                                    }
                                    break;
                                }
                            });
                        },
                        message_id::Id_Message_Undefined => {
                            //may be user is in chat room
                            if self.manager.location_mgr.current() == manager::LOCATION_CHATROOM {
                                log!("room trace1");
                                let msg = message::room_message::RoomMessage::new(
                                    message::room_message::RoomEventMsg,
                                    &self.manager.get_name(),
                                    &self.manager.location_mgr.get_room_name(),
                                    &input
                                );
                                log!("room trace2");
                                self.socket.get_output_stream().write(msg.to_bytes().as_slice());
                            }
                        }
                        _=>{}
                    }
                },
                Err(_) => {}
            }
            input.clear();
        }
    }
}
