mod lang;
mod concurrent;
mod io;
mod testsuite;
mod tools;
mod security;
mod net;
mod coroutine;
mod time;
mod process;
mod chatroom;

use crate::chatroom::entry;

fn main() {
    //for arg in std::env::args() {
    //    println!("arg is {}",arg);
    //}
    
    //chatroom::test::test_util_log::test_log();

    let args:Vec<String> = std::env::args().collect();
    let chat_type:u32 = args.get(1).unwrap().parse().unwrap();
    entry::chat_room_entry(chat_type);
}
