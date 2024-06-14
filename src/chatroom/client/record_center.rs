use crate::io::{TarFile, TarInputFileStream, TarIoStream};
use crate::lang::system;
use crate::io::filestream::{TarFileOutputStream};
use crate::io::reader::TarTextLineReader;

use std::sync::Mutex;
use std::collections::HashMap;

pub const SPLIT_STR:&str = "/";
pub const CON_STR:&str = "|#_1+F_#}";
pub const WRAP:&str = "\r\n";
pub struct ChatRecord {
    from:String,
    msg:String,
    time:u64
}

pub struct PathManager {
    base_path:String,
    chat_history_dir_path:String,
}

impl PathManager {
    pub fn set_base_path(&mut self,path:&String) {
        self.base_path = path.clone();
    }

    pub fn set_chat_history_dir_path(&mut self,path:&String) {
        self.chat_history_dir_path = path.clone();
    }

    pub fn get_base_path(&self)->String {
        self.base_path.clone()
    }

    pub fn get_chat_history_dir_path(&self)->String {
        self.chat_history_dir_path.clone()
    }
}
pub struct RecordCenter {
    path_mgr:Mutex<PathManager>,
    streams:Mutex<HashMap<String,TarFileOutputStream>>
}

impl RecordCenter {
    pub fn new()->Self {
        RecordCenter {
            path_mgr:Mutex::new(PathManager {
                base_path:String::from(""),
                chat_history_dir_path:String::from(""),
            }),
            streams:Mutex::new(HashMap::new())
        }
    }

    pub fn update_user_name(&self,name:&String) {
        let chat_history_dir_path = name.clone() + &String::from("/chat_history");
        let chat_history = TarFile::new(chat_history_dir_path.clone());
        chat_history.create_dirs();

        let mut lock = self.path_mgr.lock().unwrap();
        lock.set_base_path(name);
        lock.set_chat_history_dir_path(&chat_history_dir_path);
    }

    pub fn get_chat_history_path(&self,user_name:&String)->String {
        let lock = self.path_mgr.lock().unwrap();
        lock.get_chat_history_dir_path() + "/" + user_name
    }

    pub fn add_chat_record(&self,user:&String,who:&String,msg:&String) {
        let path = self.get_chat_history_path(user);

        let mut lock = self.streams.lock().unwrap();
        if !lock.contains_key(user) {
            //create file
            let dir = TarFile::new(path.clone());
            dir.create_dirs();
            let file = TarFile::new(path + SPLIT_STR + "/history.txt");
            if !file.exists() {
                file.create_new_file();
            }

            let output = TarFileOutputStream::new_append_stream(&file);
            lock.insert(user.clone(), output);
        }

        let stream = lock.get_mut(user).unwrap();
        let content = who.clone() + CON_STR + msg + CON_STR + &system::current_millis().to_string() + WRAP;
        stream.write(content.as_bytes());
    }

    /*
     *return value(from,message,date)
     */
    pub fn get_chat_history(&self,name:&String)->Vec<(String,String,u64)> {
        let path = self.get_chat_history_path(name) + "/history.txt";
        let mut list:Vec<(String,String,u64)> = Vec::new();
        let mut reader = TarTextLineReader::new(&path);

        loop {
            let read_result = reader.read_line();
            match read_result {
                Ok(content)=> {
                    //who
                    let mut m_who = String::from("");
                    let mut m_content= String::from("");
                    let mut m_time:u64 = 0;
                    let who_ret = content.find(CON_STR);
                    if let Some(who_index) = who_ret {
                        m_who = content[0..who_index].to_string();
                        let rest = &content[who_index + CON_STR.len()..];
                        let content_ret = rest[0..].find(CON_STR);
                        if let Some(content_index) = content_ret {
                            m_content = rest[0..content_index].to_string();
                            m_time = rest[content_index + CON_STR.len()..rest.len() - 2].to_string().parse().unwrap();
                            list.push((m_who,m_content,m_time));
                        }
                    }

                },
                Err(_)=> {
                    break;
                }
            }
        }

        return list;
    }
}
