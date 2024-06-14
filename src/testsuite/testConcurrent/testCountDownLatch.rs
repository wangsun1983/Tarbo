use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::concurrent::countdownlatch::TarCountDownLatch;

pub fn do_test1() {
    let c = Arc::new(TarCountDownLatch::new(2));
    let closure_c1 = c.clone();
    
    thread::spawn(move || {
        println!("thread1 start");
        thread::sleep(Duration::from_secs(2));
        println!("thread1 trace1");
        //let mut v = closure_c1.lock().unwrap();
        println!("thread1 trace2");
        closure_c1.count_down();
        println!("thread1 finish");
    });

    let closure_c2 = c.clone();
    thread::spawn(move || {
        println!("thread2 start");
        thread::sleep(Duration::from_secs(3));
        println!("thread2 trace1");
        //let mut v = closure_c2.lock().unwrap();
        println!("thread2 trace2");
        closure_c2.count_down();
        println!("thread2 finish");
    });

    println!("main start wait");
    c.clone().await_forever();
    println!("main finish wait");
}