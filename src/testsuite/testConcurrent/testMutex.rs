use crate::concurrent::mutex::{self, TarMutex,};
use crate::lang::{self, system};

use crate::TarAutolock;

use std::alloc::System;
use std::{sync::Arc, time::Duration};

pub fn test_mutex_1() {
    let t = Arc::new(TarMutex::new());
    println!("main trace1");
    t.lock();
    println!("main trace2");
    t.lock();
    println!("main trace3");

    let t1 = t.clone();
    let h1 = std::thread::spawn(move|| {
        println!("thread trace1");
        t1.lock();
        println!("thread trace2");
    });

    std::thread::sleep(Duration::from_secs(1));
    t.unlock();
    std::thread::sleep(Duration::from_secs(1));
    t.unlock();
    println!("main trace4,release all");


    h1.join();
    println!("main trace5");
}

pub fn test_mutex_2() {
    let t = Arc::new(TarMutex::new());
    let t1 = t.clone();
    let t2 = t.clone();

    let h1 = std::thread::spawn(move || {
        //t1.lock();
        TarAutolock!(l,t1);
        TarAutolock!(l2,t1);
        println!("t1 start", );
        std::thread::sleep(Duration::from_secs(2));
        println!("t1 finish", );
    });

    std::thread::sleep(Duration::from_millis(100));
    
    let h2 = std::thread::spawn(move || {
        println!("t2 start", );
        TarAutolock!(l,t2);
        println!("t2 trace1", );
        std::thread::sleep(Duration::from_secs(2));
        println!("t2 finish", );
    });

    h1.join();
    h2.join();
}

pub fn test_mutex_timeout_1() {
    let t = Arc::new(TarMutex::new());
    let t1 = t.clone();
    let t2 = t.clone();

    let h1 = std::thread::spawn(move || {
        //t1.lock();
        println!("t1 start", );
        TarAutolock!(l,t1);
        println!("t1 trace1", );
        std::thread::sleep(Duration::from_secs(2));
        println!("t1 finish", );
    });

    std::thread::sleep(Duration::from_millis(100));
    println!("main thread start at {}", system::current_millis());
    t2.lock_timeout(1000);
    println!("main thread trace1 at {}", system::current_millis());

    h1.join();
}