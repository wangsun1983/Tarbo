use crate::chatroom::server::server;
use crate::chatroom::client::client;

pub fn chat_room_entry(chat_type:u32) {
    match chat_type {
        1 => {
            let mut server = server::ChatServer::new(String::from("127.0.0.1"),1234);
            server.start();
            server.process_command();
        }
        2 => {
            let mut client = client::ChatClient::new();
            client.process_command();
        }
        _=>{}
    }
}