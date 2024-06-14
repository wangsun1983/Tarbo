use crate::tools::stringhelper;
pub fn testString_case1() {
    let mut data:[u8;32] = [0;32];
    data[0] = 'a' as u8;
    
    let my_str1 = stringhelper::to_string(&data);
    println!("my_str1 len is {},my_str1 is {}",my_str1.len(),my_str1);

    let mut data2:[u8;32] = [0;32];
    let my_str2 = stringhelper::to_string(&data2);

    println!("my_str2 len is {},my_str2 is {}",my_str2.len(),my_str2);

}