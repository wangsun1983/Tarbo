use std::sync::Arc;
use std::time::Duration;

use crate::concurrent::condition::TarCondition;
use crate::concurrent::TarMutex;

use crate::lang::system;

pub fn test_condition_1() {
    let mutex = Arc::new(TarMutex::new());
    let cond = Arc::new(TarCondition::new());

    let m1: Arc<TarMutex> = mutex.clone();
    let m2 = mutex.clone();

    let cond1 = cond.clone();
    let cond2 = cond.clone();
    let cond3 = cond.clone();

    let h1 = std::thread::spawn(move ||{
        println!("start write thread1", );
        std::thread::sleep(Duration::from_secs(1));
        println!("start write thread2", );
        cond1.notify_all();
        println!("start write thread3", );
    });

    let h2 = std::thread::spawn(move ||{
        println!("start1 wait at {}",system::current_millis());
        m1.lock();
        cond2.wait(m1.clone());
        m1.unlock();
        println!("finish1 wait at {}",system::current_millis());
    });

    let h3 = std::thread::spawn(move ||{
        println!("start2 wait at {}",system::current_millis());
        m2.lock();
        cond3.wait(m2.clone());
        m2.lock();
        println!("finish2 wait at {}",system::current_millis());
    });

    h1.join();
    h2.join();
    h3.join();
}