use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::thread;

pub fn current_millis()->u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()*1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

pub fn my_tid()->thread::ThreadId {
    thread::current().id()
}