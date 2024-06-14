use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::concurrent::handler;
use crate::concurrent::handler::TarHandlerThread;
use crate::concurrent::handler::TarMessage;
use crate::concurrent::handler::TarMessageQueue;
use crate::lang::system;

pub fn start_test() {
    test_message_queue_enqueue();
}

pub fn test_message_queue_enqueue() {
    let mut queue = TarMessageQueue::new();
    // queue.enqueue_message(Message::new(1));
    // queue.enqueue_message(Message::new(2));

}

pub fn test_handler_construct() {
    let mut t: TarMessage = TarMessage::new(1);
}

struct MyProcessor {
    name:String,
}

// impl TarProcessMessage for MyProcessor {
//     fn handleMessage(&self,msg:TarMessage) {
//         //println!("msg is {},thread id is {:?}",msg.what,system::myTid());
//         println!("[name is {}],msg is {},threadid is {:?}",self.name,msg.what,system::my_tid());
//     }
// }

pub fn test_handler_send_empty_message() {
    println!("test_handler_send_empty_message trace1");
    let mut handle_thread = handler::TarHandlerThread::new();
    handle_thread.start();
    //thread::sleep(Duration::from_millis(100));

    let h = handler::TarHandler::new(Box::new(|msg:TarMessage| {
        println!("[name is {}],msg is {},threadid is {:?}","first_handler",msg.what,system::my_tid());
    }));

    println!("test_handler_send_empty_message trace2");
    h.send_empty_message(1);
    h.send_empty_message_delayed(2,1000);

    println!("test_handler_send_empty_message trace3");
    let h2 = handler::TarHandler::new_with_looper(Box::new(
        |msg:TarMessage|{
            println!("[name is {}],msg is {},threadid is {:?}","second_handler",msg.what,system::my_tid());
        }), handle_thread.get_looper());
        
    h2.send_empty_message(1);
    println!("test_handler_send_empty_message trace4");
    h2.send_empty_message_delayed(2,1000);
    println!("test_handler_send_empty_message trace5");
    h.remove_messages(2);
    thread::sleep(Duration::from_secs(100));
}

pub fn test_handler_quit() {
    let h = handler::TarHandler::new(Box::new(
        |msg:TarMessage|{
            println!("[name is {}],msg is {},threadid is {:?}","second_handler",msg.what,system::my_tid());
        }
    ));
    thread::sleep(Duration::from_secs(1));
    h.quit();

    h.send_empty_message(1);
    thread::sleep(Duration::from_micros(1));
}

pub fn test_handler_quit_with_looper() {
    let mut handle_thread = TarHandlerThread::new();
    handle_thread.start();
    
    let h = handler::TarHandler::new_with_looper(Box::new(
        |msg:TarMessage|{
            println!("[name is {}],msg is {},threadid is {:?}","first_handler",msg.what,system::my_tid());
        }
    ), handle_thread.get_looper());
    h.send_empty_message(1);
    println!("test_handler_send_empty_message trace4");
    h.send_empty_message_delayed(2,1000);

    h.quit();
    let h2 = handler::TarHandler::new_with_looper(Box::new(
        |msg:TarMessage|{
            println!("[name is {}],msg is {},threadid is {:?}","second_handler",msg.what,system::my_tid());
        }), handle_thread.get_looper());
    h2.send_empty_message_delayed(1,2000);
   
    thread::sleep(Duration::from_secs(3));
}