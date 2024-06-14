use ansi_term::Colour::{Blue,Green,Red,Yellow,Purple};

use crate::chatroom::client::manager::PendingTask;
use crate::chatroom::message::message_id;
use crate::time::calendar::TarCalendar;
use crate::time::datetime;
use crate::time::calendar;

pub fn display_error(text:&String) {
    println!("{} {}",Red.bold().paint("[ERROR]"),Purple.paint(text));
}

pub fn display_hisotry_record(from:&String,text:&String,time:u64) {
    let c = TarCalendar::new(time);
    let time_str = c.get_date_time().to_string_ISO8601();

    println!("{} {} :{} {}",Blue.bold().paint("[HISTORY]"),Green.paint(from),Yellow.paint(text),time_str);
}

pub fn display_room_message(who:&String,room:&String,message:&String) {
    //println!("{} {} :{}",Blue.bold().paint("[ROOM MESSAGE]"),Green.paint(who),Yellow.paint(message));
    //[Room(abc)][Who]
    println!("{}{}{} {} :{}",Blue.bold().paint("[ROOM("),
                            Blue.bold().paint(room),
                            Blue.bold().paint(")]"),
                            Green.paint(who),
                            Yellow.paint(message));
}

pub fn display_one_message(from:&String,text:&String) {
    println!("{} {} :{}",Blue.bold().paint("[MESSAGE]"),Green.paint(from),Yellow.paint(text));
}

pub fn display_one_broadcast(from:&String,text:&String) {
    println!("{} {} :{}",Blue.bold().paint("[BROADCAST]"),Green.paint(from),Yellow.paint(text));
}

pub fn display_list(tag:&String,list:&Vec<String>) {
    println!("{}",Green.paint("=========================="));
    for user in list {
        println!("{}{}{}:{}",Blue.bold().paint("["),Blue.bold().paint(tag),Blue.bold().paint("]"),Purple.paint(user));
    }
    println!("{}",Green.paint("=========================="));
}

pub fn display_pending_task(task:&PendingTask,show_process:bool) {
    match task.get_message_id() {
        message_id::Id_FileTransfer => {
            let from = task.get_from();
            let file_name = task.get_file_name();
            let file_size = task.get_file_size();

            let file_name = task.get_file_name();
            let size = file_size/1024*1024;
            let mut size_content = String::from("");
            if size > 0 {
                size_content = size.to_string() + &String::from("M");
            } else {
                size_content = size.to_string() + &String::from("Kb");
            }

            let content = String::from("Receive file [") + &file_name + "(" + &size_content + ")] from";
            if !show_process {
                println!("{}{}{} {} {} {}",Blue.bold().paint("["),Blue.bold().paint("Notice"),Blue.paint("]"),Purple.bold().paint(content),Green.paint(from),Yellow.paint("Y/N"));
            } else {
                //println!("{}{}{} {} {} {}",Blue.paint("["),Blue.paint("Notice"),Blue.paint("]"),Purple.paint(content),Green.paint(from),Yellow.paint("Y/N"));
                let task_info = String::from("[TASK ") + &task.get_pending_id().to_string() + "]";

                match task.get_status() {
                    PendingTaskWaitingConfirm =>{
                        println!("{} {} {}",Blue.bold().paint(task_info),
                                            Purple.paint(content),
                                            Green.paint("Waiting For Confirmation"));
                    },
                    PendingTaskTransfering => {
                        println!("{} {} {} {}",Blue.bold().paint(task_info),
                                            Purple.paint(content),
                                            Green.paint("Process:"),
                                            Green.paint(task.get_process().to_string()));
                    }
                    _=> {}
                }
            }
        },
        _ => {}
    }
}