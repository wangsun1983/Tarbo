use crate::io::file::TarFile;
use crate::io::filestream::TarFileOutputStream;
use crate::io::stream::TarIoStream;

pub fn start_test() {
    //start_write_buff_append();
    //start_write_buff_truncate();
    start_seek_to_oversize();
}

fn start_write_buff_append() {
    let file = TarFile::new(String::from("write.file"));
    file.create_new_file();
    let mut output = TarFileOutputStream::new_append_stream(&file);
    let data = [0x11,0x12,0x13];
    output.write(&data).unwrap();
}

fn start_write_buff_truncate() {
    let file = TarFile::new(String::from("write.file"));
    file.create_new_file();
    let mut output = TarFileOutputStream::new_truncate_stream(&file);
    let data = [0x11,0x12,0x13];
    let size = output.write(&data).unwrap();
    println!("size is {}",size );
}

fn start_seek_to_oversize() {
    let file = TarFile::new(String::from("write.file"));
    file.create_new_file();
    let mut output = TarFileOutputStream::new_truncate_stream(&file);
    output.seek_to(100);
    let data = [0x11,0x12,0x13];
    let size = output.write(&data).unwrap();
    println!("size is {}",size );
}
