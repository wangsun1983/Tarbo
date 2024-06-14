use crate::io::reader::TarTextLineReader;

pub fn start_test() {
    test_read_line();
}

pub fn test_read_line() {
    println!("test_read_line trace1", );
    let mut reader = TarTextLineReader::new(&String::from("./test_lines.txt"));
    loop {
        match reader.read_line() {
            Ok(content) => {
                println!("data is {}",content);
            },
            Err(_)=> {
                return;
            }
        }
    }
}