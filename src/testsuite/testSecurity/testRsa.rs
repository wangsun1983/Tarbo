use crate::io::file::TarFile;
use crate::io::stream::TarIoStream;
use crate::security::rsa::{TarTransformer};
use crate::io::filestream::TarFileOutputStream;

pub fn do_test1() {
    TarTransformer::gen_key(String::from("./"));
    println!("do_test1 trace1");
    let pub_rsa = TarTransformer::new_with_pub_key(String::from("./id_rsa.pub"));
    let data = pub_rsa.encrypt_file(&String::from("./aaaa.txt"));
    println!("do_test1 trace2");
    let priv_rsa = TarTransformer::new_with_priv_key(String::from("./id_rsa"));
    let result = priv_rsa.decrypt_data(&data);
    println!("do_test1 trace3");
    let mut stream = TarFileOutputStream::new_truncate_stream(&TarFile::new(String::from("./bbb.txt")));
    stream.write(&result);
}