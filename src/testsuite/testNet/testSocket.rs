use std::thread;
use std::sync::Arc;
use std::time::Duration;
use crate::net::{address::TarAddress, socket::{self, TarSocketBuilder}};

pub fn test_connect1() {
    let builder = Arc::new(TarSocketBuilder::new());
    let builder_closure = builder.clone();
    let t1 = thread::spawn(move||{
        let socket = builder_closure.create_socket(TarAddress::new("127.0.0.1",1234));
        let client = socket.accept();
        match client {
            Some(c)=> {
                println!("one client", );
            },
            None=> {
                println!("no client", );
            }
        }
    });

    thread::sleep(Duration::from_millis(100));
    let builder_closure2 = builder.clone();
    let t2 = thread::spawn(move||{
        let mut socket = builder_closure2.create_socket(TarAddress::new("127.0.0.1",1234));
        let client = socket.connect();
        println!("connect result is {}",client);
    });

    t1.join();
    t2.join();
}

pub fn test_connect2() {
    let builder = Arc::new(TarSocketBuilder::new());
    let builder_closure = builder.clone();
    let t1 = builder.get_fila().spawn(async move {
        let mut socket = builder_closure.create_socket(TarAddress::new("127.0.0.1",1234));
        socket.bind_async().await;
        let client = socket.accept_async().await;
        match client {
            Some(c)=> {
                println!("one client", );
                let rx = c.get_input_stream();
                let tx = c.get_output_stream();
                let data = rx.read_async().await.unwrap();
                println!("read data is {}",String::from_utf8(data).unwrap());
            },
            None=> {
                println!("no client", );
            }
        }
    });

    thread::sleep(Duration::from_millis(100));
    let builder_closure2 = builder.clone();
    let t2 = builder.get_fila().spawn(async move {
        let mut socket = builder_closure2.create_socket(TarAddress::new("127.0.0.1",1234));
        let client = socket.connect_async().await;
        //println!("connect result is {}",client);
        let rx: Arc<crate::net::socketinputstream::TarSocketInputStream> = socket.get_input_stream();
        let tx = socket.get_output_stream();
        let str = String::from("hello");
        tx.write_async(str.as_bytes()).await;
    });

    std::thread::sleep(Duration::from_secs(1000))
}