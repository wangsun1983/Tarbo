use crate::security::aes;

pub fn test_case1() {
    let key = [0x1,0x2,0x34,0x5,0x6,0x7];
    aes::TarTransformer::gen_key(&key,String::from("./key"));

    let m_aes = aes::TarTransformer::new_with_default(String::from("./key"));
    let v1 = [0xa,0xb,0xa,0xb];
    let v2 = m_aes.encrypt_data(&v1);
    for v in &v2 {
        println!("enc v is {}",v);
    }

    let v3 = m_aes.decrypt_data(&v2);
    for v in &v3 {
        println!("dec v is {}",v);
    }
}