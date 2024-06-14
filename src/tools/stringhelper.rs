// pub fn is_empty(content:&String)->bool {
//     return content.is_empty() || content.as_bytes()[0] == 0;
// }

pub fn to_string(data:&[u8])->String {
    let ret = data.iter().position(|&value| value == 0);
    match ret {
        Some(index) => {
            if index == 0 {
                return String::from("");
            } else {
                return String::from_utf8(data[0..index].to_vec()).unwrap();
            }
        }
        None => {
            return String::from_utf8(data.to_vec()).unwrap();
        }
    }
}