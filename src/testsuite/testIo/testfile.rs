use crate::io::file::TarFile;

//test file
pub fn start_test() {
    //test_file_get_name();
    //test_file_get_absolute_patch();
    //test_file_exists();
    //test_is_dir();
    //test_size();
    //test_create_file();
    test_remove_file();
}

fn test_file_get_name() {
    println!("testFileGetName start");
    let f = TarFile::new(String::from("abc/aa.c"));
    println!("name is {}",f.get_name().unwrap());

    let f1 = TarFile::new(String::from("abc/abcde.caf"));
    println!("suffix is {}",f1.get_suffix().unwrap());
}

fn test_file_get_absolute_patch() {
    let f = TarFile::new(String::from("aa/Cargo.toml"));
    println!("absolute path is {}",f.get_absolute_patch().unwrap());

    let f2 = TarFile::new(String::from("/home/test/wangsun/mysource/Tarbo/src/Cargo.toml"));
    println!("absolute path is {}",f2.get_absolute_patch().unwrap());

}

fn test_file_exists() {
    let f1 = TarFile::new(String::from("Cargo.toml"));
    println!("cargo.toml {}",f1.exists());

    let f2 = TarFile::new(String::from("/home/test/wangsun/mysource/Tarbo/src/Cargo.toml"));
    println!("cargo.toml {}",f2.exists());

    let f3 = TarFile::new(String::from("aabb.cc"));
    println!("cargo.toml {}",f3.exists());
}

fn test_is_dir() {
    let f1 = TarFile::new(String::from("Cargo.toml"));
    println!("dir {}",f1.is_dir());

    let f2 = TarFile::new(String::from("./src"));
    println!("dir {}",f2.is_dir());

    let f3 = TarFile::new(String::from("./src123"));
    println!("dir {}",f3.is_dir());
}

fn test_size() {
    let f1 = TarFile::new(String::from("Cargo.toml"));
    println!("size is {}",f1.size().unwrap());
}

fn test_create_file() {
    let f1 = TarFile::new(String::from("abc.cc")).create_new_file();
    println!("create f1 is {}",f1);

    let f2 = TarFile::new(String::from("abc.cc")).create_new_file();
    println!("create f1 is {}",f2);
}

fn test_remove_file() {
    let f1 = TarFile::new(String::from("abc.cc")).create_new_file();
    
    let file = TarFile::new(String::from("abc.cc"));
    if !file.exists() {
        println!("test_remove_file,create data fail");
        return;
    }

    let f2 = file.remove_file();
    println!("remove result is {}",f2 );
}