use std::fs::{self, File};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::env;

use crate::io::filestream;
use crate::io::stream::TarIoStream;
use crate::io::filestream::TarInputFileStream;

pub struct TarFile {
    path:String
}

impl TarFile {
    pub fn new(path:String)->Self {
        TarFile {
            path:path,
        }
    }

    pub fn get_name(&self)->Option<String> {
        let names:Vec<&str> = self.path.split('/').collect();
        let mut index = names.len() - 1;
        
        while index > 0 {
            if names[index].len() != 0 {
                return Some(String::from(names[index]));
            }
            index -= 1;
        }

        if self.path.contains("/") {
            return Some(String::from("/"));
        }
        
        None
    }

    pub fn get_suffix(&self)->Option<String> {
        let result = self.get_name();
        match result {
            Some(name) => {
                for(index,ch) in name.chars().rev().enumerate() {
                    if ch == '.' {
                        println!("index is {}",index);
                        let len = name.len();
                        return Some(String::from(&name[(len - index)..len]));
                    }
                }
            },
            None => {}
    
        }
        
        return None;
    }

    pub fn get_name_with_no_suffix(&self)->Option<String> {
        let result = self.get_name();
        match result {
            Some(name) => {
                let index = name.rfind(".");
                match index {
                    Some(r_index) => {
                        return Some(String::from(&name[0..r_index]));
                    },
                    None=> {}
                }
            },
            None => {}
        }

        None
    }

    pub fn get_absolute_patch(&self)->Option<String> {
        println!("get_absolute_patch trace1");
        let mut abs = env::current_dir().unwrap();
        abs.push(&self.path);
        Some(String::from(abs.to_str().unwrap()))

        //if we can only get existed file's absolute path,use the following code
        // if let Ok(abs_path) = env::current_dir().and_then(|mut path| {
        //         path.push(&self.path);
        //         println!("path is {}",String::from(path.to_str().unwrap()));

        //         match Path::new(&path).canonicalize() {
        //             Ok(p) =>  {
        //                 println!("get_absolute_patch trace2");
        //                 return Ok(Some(p))
        //             },
        //             Err(_) => {
        //                 println!("get_absolute_patch trace3");
        //                 Ok(None)
        //             }
        //         }
        //     }) {
        //         match abs_path {
        //             Some(r_path) => {
        //                 println!("get_absolute_patch trace4");
        //                 return Some(String::from(r_path.to_str().unwrap()));
        //             },
        //             None => {
        //                 println!("get_absolute_patch trace5");
        //             }
        //         }
        // }
        //None
    }

    pub fn exists(&self)->bool {
        Path::new(&self.path).exists()
    }

    pub fn is_dir(&self) ->bool {
        Path::new(&self.path).is_dir()
    }

    pub fn is_file(&self) ->bool {
        Path::new(&self.path).is_file()
    }

    pub fn size(&self)->Option<u64> {
        let result = fs::metadata(Path::new(&self.path));
        match result {
            Ok(meta_data) => {
                return Some(meta_data.size());
            },
            Err(_) => {

            }
        }
        None
    }

    pub fn create_new_file(&self)->bool {
        let path = Path::new(&self.path);

        if path.exists() {
            return false;
        }

        match File::create(path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {}
        }

        false
    }

    pub fn remove_file(&self)->bool {
        let path = Path::new(&self.path);
        match fs::remove_file(path) {
            Ok(_) => {
                return true;
            },
            Err(_) =>{}
        }

        return false;
    }

    pub fn getInputStream(&self)->Option<Box<dyn TarIoStream>> {
        if self.exists() {
            return Some(Box::new(TarInputFileStream::new(&self)));
        }

        None
    }

    pub fn create_dirs(&self) {
        fs::create_dir_all(&self.path);
    }
}