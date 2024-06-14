use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::cell::RefCell;
use tokio::io::AsyncWriteExt;

use crate::net::address::TarAddress;
use crate::net::socketmonitor::{self};
use crate::net::socketmonitor::TarListener;
use crate::net::socket::{TarSocket, TarSocketBuilder};

struct client_listener{}
impl TarListener for client_listener {
    fn on_event(&mut self,event:socketmonitor::Event,data:Option<Vec<u8>>,sock:Arc<TarSocket>) {
        match event {
            socketmonitor::Event::Connect=>{
                let ss = sock.clone().get_output_stream();
                thread::spawn(move || {
                    println!("start thread trace1", );
                    loop {
                        println!("start thread trace2", );
                        ss.write("helloworld".as_bytes());
                        thread::sleep(Duration::from_secs(1));
                        println!("start thread trace3", );
                    }
                });
                
                //return Some(Vec::from("helloworld".as_bytes()));
            },
            socketmonitor::Event::Message=>{
                println!("connected succefully trace2", );
            },
            socketmonitor::Event::Disconnect=>{},
            socketmonitor::Event::NewClient=>{},
        }
    }
}

struct server_listener{}

impl TarListener for server_listener {
    fn on_event(&mut self,event:socketmonitor::Event,data:Option<Vec<u8>>,stream:Arc<TarSocket>) {
        match event {
            socketmonitor::Event::Connect=>{
                
            },
            socketmonitor::Event::Message=>{
                let str = String::from_utf8(data.unwrap()).unwrap();
                println!("get message:{}",str);
            },
            socketmonitor::Event::Disconnect=>{},
            socketmonitor::Event::NewClient=>{
                println!("new client", );
            },
        }
    }
}


pub fn test_monitor_1() {
    let t1 = thread::spawn(||{
        let monitor = socketmonitor::TarSocketMonitor::new();
        let server_socket = Arc::new(TarSocketBuilder::new().create_socket(TarAddress::new("127.0.0.1",1234)));
        monitor.bind_as_server(server_socket, Box::new(server_listener{}));
        loop {
            thread::sleep(Duration::from_secs(100));
        }
    });

    thread::sleep(Duration::from_millis(100));

    let t2 = thread::spawn(||{
        let monitor = socketmonitor::TarSocketMonitor::new();
        let mut client_socket = TarSocketBuilder::new().create_socket(TarAddress::new("127.0.0.1",1234));
        client_socket.connect();
        let client = Arc::new(client_socket);
        monitor.bind_as_client(client.clone(), Box::new(client_listener{}));
        
        let ss = client.clone().get_output_stream();
        thread::spawn(move || {
            println!("start thread trace1", );
            loop {
                println!("start thread trace2", );
                ss.write("helloworld".as_bytes());
                thread::sleep(Duration::from_secs(1));
                println!("start thread trace3", );
            }
        });
        
        loop {
            thread::sleep(Duration::from_secs(100));
        }
    });

    t1.join();
    t2.join();
}