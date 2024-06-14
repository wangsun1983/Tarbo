use crate::security::sha::{self, Tar256Transformer};

pub fn test_case1() {
    println!("sha256 is {}",Tar256Transformer::encrypt_string(&String::from("hello")));
    //sha = Tar256Transformer::new();
    println!("sha256 is {}",Tar256Transformer::encrypt_string(&String::from("hello")));
    //sha = Tar256Transformer::new();
    println!("sha256 is {}",Tar256Transformer::encrypt_string(&String::from("hello")));
}

pub fn test_case2() {
    let path = String::from("/home/test/wangsun/mysource/Tarbo/src/tarbo.tar.gz");
    println!("test_case2,file {}",Tar256Transformer::encrypt_file(&path));
}