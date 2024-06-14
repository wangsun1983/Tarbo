use std::io;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::Mutex;

use libc::write;
use tokio::sync::broadcast;

use crate::chatroom::message::broadcast_message;
use crate::chatroom::message::chatter_list_message;
use crate::chatroom::message::file_transfer_message;
use crate::chatroom::message::room_message;
use crate::chatroom::message::user_exit_message;
use crate::chatroom::server::client_manager;
use crate::net::{TarSocket, TarSocketMonitor,TarSocketBuilder,TarAddress,TarListener,Event};
use crate::lang::TarByteRingArray;
use crate::concurrent::{TarExecutor, TarMutex,TarThreadPoolExecutor,TarBlockingQueue};
use crate::TarAutolock;
use crate::tools::stringhelper;

use crate::chatroom::message::{self, chat_message, new_comer_message,message_parser};
use crate::chatroom::message::{message_id,common_ack_message};
use crate::log;

use super::client_manager::ClientManager;
use super::client_buff_manager::ClientBuffManager;
use super::room_manager;

struct Task {
    tag:String,/*  */
    id:u32,
    data:Vec<u8>,
    sock:Option<Arc<TarSocket>>
}

impl Task {
    fn new(tag:&String,id:u32,data:Vec<u8>,sock:Arc<TarSocket>)->Self {
        Task {
            tag:tag.clone(),
            id:id,
            data:data,
            sock:Some(sock.clone())
        }
    }
}
struct ChatServerManager {
    buffer_manager:ClientBuffManager,
    client_manager:Arc<Mutex<ClientManager>>,
    executor_manager:TarThreadPoolExecutor,
    task_pool:Arc<TarBlockingQueue<Task>>
}

impl ChatServerManager {
    fn new()->Self {
        let task_pool = Arc::new(TarBlockingQueue::<Task>::new());
        let client_manager = Arc::new(Mutex::new(ClientManager::new()));
        let mut manager = ChatServerManager {
            buffer_manager:ClientBuffManager::new(),
            client_manager:client_manager.clone(),
            executor_manager:TarThreadPoolExecutor::new(8),
            task_pool:task_pool.clone()
        };

        let room_manager = Arc::new(room_manager::RoomManager::new());

        for _ in 0..8 {
            let pool = task_pool.clone();
            let c_manager: Arc<Mutex<ClientManager>> = client_manager.clone();
            let r_manager = room_manager.clone();

            manager.executor_manager.submit(move||{
                loop {
                    let task = pool.takeFirst();
                    match task.id {
                        message_id::Id_RoomMessage => {
                            let (mut room_message,_)= room_message::RoomMessage::from_bytes(task.data.as_slice());
                            match room_message.get_event() {
                                room_message::RoomEventCreate=> {
                                    let lock = c_manager.lock().unwrap();
                                    let tag = task.sock.clone().unwrap().get_addr_string();
                                    let owner = lock.user_tag_to_name(&tag);
                                    let result = r_manager.create_room(&room_message.get_room_name(),&owner);
                                    drop(lock);

                                    let ack = common_ack_message::CommonAckMessage::new(message_id::Id_RoomMessage,result);
                                    let sock: Arc<TarSocket> = task.sock.clone().unwrap();
                                    sock.get_output_stream().write(&ack.to_bytes());
                                },
                                room_message::RoomEventEnter=> {
                                    let user_name = room_message.get_content();
                                    r_manager.on_user_enter_room(&room_message.get_room_name(), &user_name);
                                    //send messsage to all client
                                    let msg = room_message.to_bytes();
                                    let lock = c_manager.lock().unwrap();
                                    lock.userinfo_foreach(|user|{
                                        if !user.get_name().eq(&user_name) {
                                            user.get_socket().get_output_stream().write(&msg);
                                        }
                                    });
                                },
                                room_message::RoomEventErase=> {
                                    //TODO
                                },
                                room_message::RoomEventLeave=> {
                                    let user_name = room_message.get_content();
                                    r_manager.on_user_leave_room(&room_message.get_room_name(), &user_name);
                                    //send messsage to all client
                                    let msg = room_message.to_bytes();
                                    let lock = c_manager.lock().unwrap();
                                    lock.userinfo_foreach(|user|{
                                        if !user.get_name().eq(&user_name) {
                                            user.get_socket().get_output_stream().write(&msg);
                                        }
                                    });
                                },
                                room_message::RoomEventMsg=> {
                                    log!("server room msg trace1");
                                    let user_name = room_message.get_content();
                                    r_manager.on_user_leave_room(&room_message.get_room_name(), &user_name);
                                    //send messsage to all client
                                    let msg = room_message.to_bytes();
                                    let lock = c_manager.lock().unwrap();
                                    lock.userinfo_foreach(|user|{
                                        log!("server room msg trace2,send to user {}",user.get_name());
                                        user.get_socket().get_output_stream().write(&msg);
                                    });
                                },
                                room_message::RoomEventList=> {
                                    let rooms = r_manager.get_rooms();
                                    room_message.set_content(&rooms);
                                    task.sock.clone().unwrap().get_output_stream().write(&room_message.to_bytes());
                                },
                                room_message::RoomEventListUsers=> {
                                    let users = r_manager.get_users(&room_message.get_room_name());
                                    room_message.set_content(&users);
                                    task.sock.clone().unwrap().get_output_stream().write(&room_message.to_bytes());
                                }
                                _=> {
                                    //TODO
                                }
                            }
                        },
                        message_id::Id_BroadCast => {
                            let broadcast_msg: (broadcast_message::BroadCastMessage, usize) = broadcast_message::BroadCastMessage::from_bytes(task.data.as_slice());
                            let broadcast_bytes = broadcast_msg.0.to_bytes();
                            let lock = c_manager.lock().unwrap();
                            lock.userinfo_foreach(|info:&client_manager::ClientInfo| {
                                if info.get_name() != broadcast_msg.0.get_msg_from() {
                                    info.get_socket().get_output_stream().write(&broadcast_bytes);
                                }
                            });
                        },
                        message_id::Id_ChatMessage => {
                            let chat_msg = chat_message::ChatMessage::from_bytes(task.data.as_slice());
                            {
                                let lock = c_manager.lock().unwrap();
                                lock.send_msg(chat_msg.0);
                            }
                        },
                        message_id::Id_NewComer => {
                            let comer_msg = new_comer_message::NewComerMessage::from_bytes(task.data.as_slice());
                            let mut ack = common_ack_message::CommonAckMessage::new(message_id::Id_NewComer,message_id::Result_Ok);
                            let sock = task.sock.unwrap().clone();
                            let mut is_user_exists = false;
                            log!("server,Id_NewComer,name is {}",comer_msg.0.get_name());
                            {
                                let mut lock = c_manager.lock().unwrap();
                                if lock.add_new_client(&comer_msg.0.get_name(),&comer_msg.0.get_old_name(),sock.clone()) == message_id::Result_Already_Exist {
                                    ack.set_result(message_id::Result_Already_Exist);
                                    //TODO remove buffer manager
                                    is_user_exists = true;
                                }
                            }
                            sock.get_output_stream().write(&ack.to_bytes());
                            //if it is a new name,we should send all chatters name
                            if !is_user_exists && comer_msg.0.get_old_name().is_empty() {
                                let lock = c_manager.lock().unwrap();
                                let users: Vec<String> = lock.get_all_users_without_someone(&comer_msg.0.get_name());
                                drop(lock);
                                if users.len() > 0 {
                                    let msg = chatter_list_message::ChatterListMessage::new_with_list(&users);
                                    sock.get_output_stream().write(&msg.to_bytes());
                                }
                            }
                            //send new commer to others
                            let notify_msg = comer_msg.0.to_bytes();
                            let lock = c_manager.lock().unwrap();
                            lock.userinfo_foreach(|info:&client_manager::ClientInfo| {
                                if info.get_name() != comer_msg.0.get_name() {
                                    info.get_socket().get_output_stream().write(&notify_msg);
                                }
                            });
                        },
                        message_id::Id_UserExit => {
                            let tag = String::from_utf8(task.data).unwrap();
                            let mut lock = c_manager.lock().unwrap();
                            let client_name = lock.user_tag_to_name(&tag);
                            lock.remove_client(&tag);

                            //send notify message to all client
                            let away_message = user_exit_message::UserExitMessage::new(client_name).to_bytes();
                            lock.userinfo_foreach(|info:&client_manager::ClientInfo| {
                                info.get_socket().get_output_stream().write(&away_message);
                            });
                        },
                        message_id::Id_FileTransfer => {
                            let sock = task.sock.unwrap();
                            let mut lock = c_manager.lock().unwrap();
                            let client_name = lock.user_tag_to_name(&sock.get_addr_string());
                            drop(lock);

                            let mut file_msg = file_transfer_message::FileTransferMessage::from_bytes(task.data.as_slice()).0;
                            file_msg.set_ip(sock.get_ip());
                            file_msg.set_from(client_name);
                            {
                                let lock = c_manager.lock().unwrap();
                                lock.send_file_msg(file_msg);
                            }
                        }
                        _ => {
                            //TODO
                        },
                    }
                    
                }
            });
        }
        manager
    }
}

impl TarListener for ChatServerManager {
    fn on_event(&mut self,event:Event,data:Option<Vec<u8>>,sock:Arc<TarSocket>) {
        let tag = sock.get_addr_string();

        match event {
            Event::NewClient => {
                self.buffer_manager.add_new_client(&tag);
            },
            Event::Message => {
                if let Some(recv_data) = data {
                    self.buffer_manager.push_buff(&tag, &recv_data);
                    self.buffer_manager.process_buff(&tag, |id:u32,data:Vec<u8>| {
                        let task = Task::new(&tag, id, data, sock.clone());
                        self.task_pool.putLast(task);
                    });
                }
            },
            Event::Disconnect => {
                log!("one client disconnect!!!", );
                self.buffer_manager.remove_client(&tag); //remove buff first
                let task = Task::new(&tag, message_id::Id_UserExit, tag.as_bytes().to_vec(), sock.clone());
                self.task_pool.putLast(task);
            },
            Event::Connect => {
                //TODO
            },
        }
    }
}


pub struct ChatServer {
    monitor:TarSocketMonitor,
    port:u32,
    host_ip:String
}

impl ChatServer {
    pub fn new(ip:String,port:u32)->Self {
        ChatServer {
            monitor:TarSocketMonitor::new(),
            port:port,
            host_ip:ip
        }
    }

    pub fn start(&mut self) {
        let server_socket 
            = Arc::new(TarSocketBuilder::new().create_socket(
                    TarAddress::new(&self.host_ip,self.port)));

        self.monitor.bind_as_server(server_socket.clone(),
                     Box::new(ChatServerManager::new()));
    }

    pub fn process_command(&mut self) {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_)=> {
                    //TODO
                    log!("server you type is {}",input.trim());
                },
                Err(_) => {}
            }
        }
    }
}