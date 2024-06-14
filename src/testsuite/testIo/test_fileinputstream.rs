use crate::io::filestream::TarInputFileStream;
use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;

pub fn start_test() {
    // test_read();
    // test_seek();
    test_seek_over_size();
}

pub fn test_read() {
    let file = TarFile::new(String::from("test.txt"));
    let mut input = TarInputFileStream::new(&file);
    let mut data= [0];

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());
}

pub fn test_seek() {
    let file = TarFile::new(String::from("test.txt"));
    let mut input = TarInputFileStream::new(&file);
    let mut data= [0];

    input.seek_to(1);
    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());
}

pub fn test_seek_over_size() {
    let file = TarFile::new(String::from("test.txt"));
    let mut input = TarInputFileStream::new(&file);
    let mut data= [0];
    let result = input.seek_to(100);
    println!("result is {}",result);

    let ret = input.read(&mut data);
    println!("data is {},usize is {}",data[0],ret.unwrap());


}
