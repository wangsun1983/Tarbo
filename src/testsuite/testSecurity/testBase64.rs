use crate::security::base64;

pub fn do_test1() {
    let str = String::from("hello world");
    let encode_ret = base64::TarTransformer::encode(str.as_bytes());

    println!("encode_ret is {}",encode_ret);
    let decode_ret = base64::TarTransformer::decode(&encode_ret);
    println!("decode_ret is {}",String::from_utf8_lossy(decode_ret.as_slice()));
}