use crate::time::datetime::{self, TarDateTime};

pub fn test_ISO8601() {
    let mut datetime1 = TarDateTime::new();
    datetime1.import_iso8601("2005-01-08T12:30:12Z");
    datetime1.dump();

    let str1 = datetime1.to_string_ISO8601();
    println!("datetime is {}",str1);
}