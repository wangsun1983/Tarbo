use crate::security::crc::TarTransformer;

pub fn do_crc32_test1() {
    let key = [0x1,0x2,0x34,0x5,0x6,0x7];
    println!("{:04x}",TarTransformer::crc32_data(&key));
}

pub fn do_crc32_test2() {
    let path = String::from("/home/test/wangsun/mysource/Obotcha/ObotchaTestSuite/testSecurity/testCrc32/testData.file");
    println!("test_case2,file {}",TarTransformer::crc32_file(&path));
}

pub fn do_crc64_test3() {
    let path = String::from("/home/test/wangsun/mysource/Tarbo/src/Cargo.toml");
    println!("test_case3,file {}",TarTransformer::crc64_file(&path));
}

