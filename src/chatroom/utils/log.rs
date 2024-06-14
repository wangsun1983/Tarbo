use std::fmt::{format, Arguments};

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if false {
            println!("{}",format!($($arg)*));
        }
    };
}
