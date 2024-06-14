use crate::security::md::TarTransformer;

pub fn test_case1() {
    let path = String::from("/home/test/wangsun/mysource/Tarbo/src/tarbo.tar.gz");
    println!("test_case2,file {}",TarTransformer::encrypt_file(&path));
}