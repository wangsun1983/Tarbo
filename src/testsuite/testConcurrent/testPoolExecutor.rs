use std::{thread, time::Duration};

use crate::{concurrent::poolexecutor::{self, TarPoolExecutor, TarPriorityPoolExecutor, TarSchedulePoolExecutor}, lang::system};

pub fn test_thread_pool_1() {
    let pool = TarPoolExecutor::<u32>::new(8);
    let f1 = pool.submit(||{
        //println!("f1 start", );
        std::thread::sleep(Duration::from_secs(5));
        //println!("f1 trace", );
        123
    });

    println!("test_thread_pool_1 start");
    let v = f1.get().unwrap();
    println!("test_thread_pool_1 v is {}",v);
}

pub fn test_thread_pool_2() {
    let pool = TarPoolExecutor::<()>::new(8);
    let f1 = pool.submit(||{
        //println!("f1 start", );
        std::thread::sleep(Duration::from_secs(5));
        //println!("f1 trace", );
    });


    println!("test_thread_pool_2 start");
    f1.wait();
    println!("test_thread_pool_2");
}

pub fn test_thread_pool_3() {
    let pool = TarPoolExecutor::<()>::new(8);
    let f1 = pool.submit(||{
        println!("f1 start", );
        std::thread::sleep(Duration::from_secs(5));
        println!("f1 trace", );
    });


    println!("test_thread_pool_3 start at {}",system::current_millis());
    f1.wait_timeout(1000);
    println!("test_thread_pool_3 end at {}",system::current_millis());

    std::thread::sleep(Duration::from_millis(1000*8));
    println!("test_thread_pool_3 exit");
}

pub fn test_thread_pool_shutdown_1() {
    let pool = TarPoolExecutor::<()>::new(1);
    let f1 = pool.submit(||{
        println!("f1 start", );
        std::thread::sleep(Duration::from_secs(5));
        println!("f1 trace", );
    });

    let f2 = pool.submit(||{
        println!("f2 start", );
        std::thread::sleep(Duration::from_secs(5));
        println!("f2 trace", );
    });

    std::thread::sleep(Duration::from_secs(1));
    pool.shut_down();
    println!("wait start at {}", system::current_millis());
    pool.wait_termination();
    println!("wait end at {}", system::current_millis());
}

pub fn test_schedule_thread_pool_1() {
    let pool = TarSchedulePoolExecutor::<u32>::new(4);
    println!("test trace1 at {}",system::current_millis() );
    pool.schedule(1000,||{
        println!("start 1");
        1
    });

    println!("test trace2", );
    pool.schedule(2000,||{
        println!("start 2");
        1
    });

    println!("test trace3", );
    pool.schedule(500,||{
        println!("start 3");
        1
    });

    println!("test trace3", );
    pool.schedule(1000,||{
        println!("start 4");
        1
    });

    std::thread::sleep(Duration::from_secs(10));
}

pub fn test_schedule_thread_pool_2() {
    let pool = TarSchedulePoolExecutor::<u32>::new(4);
    println!("test trace1 at {}",system::current_millis() );
    let future = pool.schedule(1000,||{
        println!("start 1");
        123
    });

    std::thread::sleep(Duration::from_secs(5));
    let value = future.get();
    println!("value is {}", value.unwrap());
}

pub fn test_schedule_thread_pool_3() {
    let pool = TarSchedulePoolExecutor::<u32>::new(4);
    println!("test trace1 at {}",system::current_millis() );
    let future1 = pool.schedule(1000,||{
        println!("start 1");
        thread::sleep(Duration::from_secs(5));
        println!("finish 1");
        123
    });

    let future2 = pool.schedule(1000,||{
        println!("start 2");
        thread::sleep(Duration::from_secs(5));
        println!("finish 2");
        456
    });

    std::thread::sleep(Duration::from_secs(6));
    let value1 = future1.get();
    let value2 = future2.get();
    println!("value1 is {},value2 is {}", value1.unwrap(),value2.unwrap());
}

pub fn test_schedule_thread_pool_shut_down() {
    let pool = TarSchedulePoolExecutor::<u32>::new(1);
    println!("test trace1 at {}",system::current_millis() );
    let future1 = pool.schedule(500,||{
        println!("start 1");
        thread::sleep(Duration::from_secs(5));
        println!("finish 1");
        123
    });

    let future2 = pool.schedule(500,||{
        println!("start 2");
        thread::sleep(Duration::from_secs(5));
        println!("finish 2");
        456
    });

    std::thread::sleep(Duration::from_secs(1));
    pool.shut_down();
    println!("start wait at {}",system::current_millis());
    pool.wait_termination();
    println!("finish wait at {}",system::current_millis());
}

pub fn test_priority_thread_pool_1() {
    let pool = TarPriorityPoolExecutor::<u32>::new(2);
    println!("test trace1 at {}",system::current_millis() );
    let future1 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high1 start");
        std::thread::sleep(Duration::from_secs(1));
        1122
    });

    let future2 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high2 start");
        std::thread::sleep(Duration::from_secs(1));
        2233
    });

    let future3 = pool.submit(poolexecutor::TarPriority::Low, || {
        println!("low3 start");
        std::thread::sleep(Duration::from_secs(1));
        3344
    });

    let future4 = pool.submit(poolexecutor::TarPriority::Mid, || {
        println!("mid4 start");
        std::thread::sleep(Duration::from_secs(1));
        4455
    });

    let future5 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high5 start");
        std::thread::sleep(Duration::from_secs(1));
        5566
    });

    println!("value1 is {}", future1.get().unwrap());
    println!("value2 is {}", future2.get().unwrap());
    println!("value3 is {}", future3.get().unwrap());
    println!("value4 is {}", future4.get().unwrap());
    println!("value5 is {}", future5.get().unwrap());
}


pub fn test_priority_thread_pool_shut_down_1() {
    let pool = TarPriorityPoolExecutor::<u32>::new(2);
    println!("test trace1 at {}",system::current_millis() );
    let future1 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high1 start");
        std::thread::sleep(Duration::from_secs(1));
        1122
    });

    let future2 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high2 start");
        std::thread::sleep(Duration::from_secs(1));
        2233
    });

    let future3 = pool.submit(poolexecutor::TarPriority::Low, || {
        println!("low3 start");
        std::thread::sleep(Duration::from_secs(1));
        3344
    });

    let future4 = pool.submit(poolexecutor::TarPriority::Mid, || {
        println!("mid4 start");
        std::thread::sleep(Duration::from_secs(1));
        4455
    });

    let future5 = pool.submit(poolexecutor::TarPriority::High, || {
        println!("high5 start");
        std::thread::sleep(Duration::from_secs(1));
        5566
    });

    std::thread::sleep(Duration::from_millis(100));

    pool.shut_down();
    println!("start wait at {}",system::current_millis());
    pool.wait_termination();
    println!("start end at {}",system::current_millis());

}