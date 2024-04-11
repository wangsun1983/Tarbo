use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct TarTextLineReader {
    reader:BufReader<File>,
}

impl TarTextLineReader {
    pub fn new(path:&String)->Self {
        let file = File::open(path).unwrap();
        TarTextLineReader {
            reader:BufReader::new(file)
        }
    }

    pub fn read_line(&mut self)->Result<String,u32> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(size) => {
                if size > 0 {
                    return Ok(line);
                }
            },
            Err(_)=> {}
        }
        Err(1)
    }
}