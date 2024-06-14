use std::sync::RwLock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::concurrent::rwlock;
use crate::lang::system;

pub fn test_rwlock_1() {
    let rwlock = rwlock::TarRwLock::new();
    let rd = Arc::new(rwlock.acquire_rd_lock());
    let wr = Arc::new(rwlock.acquire_wr_lock());

    let rd1 = rd.clone();
    let rd2 = rd.clone();

    let wr1 = wr.clone();
    let wr2 = wr.clone();

    let h1 = std::thread::spawn(move|| {
        rd1.lock();
        println!("read thread start trace1");
        thread::sleep(Duration::from_secs(5));
        rd1.unlock();
        println!("read thread start trace2");
    });


    let h2 = std::thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        println!("write thread start at {}",system::current_millis());
        wr1.lock();
        println!("write thread start at {}",system::current_millis());
        wr1.unlock();
    });

    h1.join();
    h2.join();
}

pub fn test_rwlock_2() {
    let rwlock = rwlock::TarRwLock::new();
    let rd = Arc::new(rwlock.acquire_rd_lock());
    let wr = Arc::new(rwlock.acquire_wr_lock());

    let rd1 = rd.clone();
    let rd2 = rd.clone();

    let wr1 = wr.clone();
    let wr2 = wr.clone();

    let h1 = std::thread::spawn(move|| {
        rd1.lock();
        println!("read1 thread start trace1");
        thread::sleep(Duration::from_secs(5));
        rd1.unlock();
        println!("read1 thread start trace2");
    });


    let h2 = std::thread::spawn(move|| {
        println!("read2 thread start at {}",system::current_millis());
        rd2.lock();
        println!("read2 thread start at {}",system::current_millis());
        rd2.unlock();
    });

    h1.join();
    h2.join();
}

pub fn test_rwlock_3() {
    let rwlock = rwlock::TarRwLock::new();
    let rd = Arc::new(rwlock.acquire_rd_lock());
    let wr = Arc::new(rwlock.acquire_wr_lock());

    let rd1 = rd.clone();
    let rd2 = rd.clone();
    let rd3 = rd.clone();

    let wr1 = wr.clone();
    let wr2 = wr.clone();

    let h1 = std::thread::spawn(move|| {
        wr1.lock();
        println!("wr1 thread start trace1");
        thread::sleep(Duration::from_secs(5));
        wr1.unlock();
        println!("wr1 thread start trace2");
    });


    let h2 = std::thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        println!("read2 thread start at {}",system::current_millis());
        rd2.lock();
        println!("read2 thread start at {}",system::current_millis());
        rd2.unlock();
    });

    let h3 = std::thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        println!("read3 thread start at {}",system::current_millis());
        rd3.lock();
        println!("read3 thread start at {}",system::current_millis());
        rd3.unlock();
    });

    h1.join();
    h2.join();
    h3.join();
}

pub fn test_rwlock_timeout_1() {
    let rwlock = rwlock::TarRwLock::new();
    let rd = Arc::new(rwlock.acquire_rd_lock());
    let wr = Arc::new(rwlock.acquire_wr_lock());

    let rd1 = rd.clone();
    let rd2 = rd.clone();
    let rd3 = rd.clone();

    let wr1 = wr.clone();
    let wr2 = wr.clone();

    let h1 = std::thread::spawn(move|| {
        wr1.lock();
        println!("wr1 thread start trace1");
        thread::sleep(Duration::from_secs(5));
        wr1.unlock();
        println!("wr1 thread start trace2");
    });


    let h2 = std::thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        println!("read2 thread start at {}",system::current_millis());
        let ret = rd2.lock_timeout(1000);
        println!("read2 thread start at {},ret is {}",system::current_millis(),ret);
    });

    h1.join();
    h2.join();
}

pub fn test_rwlock_timeout_2() {
    let rwlock = rwlock::TarRwLock::new();
    let rd = Arc::new(rwlock.acquire_rd_lock());
    let wr = Arc::new(rwlock.acquire_wr_lock());

    let rd1 = rd.clone();
    let rd2 = rd.clone();
    let rd3 = rd.clone();

    let wr1 = wr.clone();
    let wr2 = wr.clone();

    let h1 = std::thread::spawn(move|| {
        rd1.lock();
        println!("rd1 thread start trace1");
        thread::sleep(Duration::from_secs(5));
        rd1.unlock();
        println!("rd1 thread start trace2");
    });


    let h2 = std::thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        println!("wr1 thread start at {}",system::current_millis());
        let ret = wr1.lock_timeout(1000);
        println!("wr1 thread start at {},ret is {}",system::current_millis(),ret);
    });

    h1.join();
    h2.join();
}