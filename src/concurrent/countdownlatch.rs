use std::sync::Mutex;
use std::sync::Condvar;
use std::time::Duration;

pub struct TarCountDownLatch {
    m_count:Mutex<u32>,
    m_cond:Condvar
}

impl TarCountDownLatch {
    pub fn new(count:u32)->Self {
        TarCountDownLatch {
            m_count:Mutex::new(count),
            m_cond:Condvar::new()
        }
    }

    pub fn count_down(&self) {
        println!("count down trace1");
        let mut lock = self.m_count.lock().unwrap();
        println!("count down trace2");
        if *lock > 0 {
            *lock -= 1;
            if *lock == 0 {
                println!("count down trace3");
                self.m_cond.notify_all();
            }
        }
        println!("count down trace4");
    }

    pub fn await_timeout(&self,interval:u64) -> i32 {
        let lock = self.m_count.lock().unwrap();
        let result = self.m_cond.wait_timeout(lock, Duration::from_millis(interval)).unwrap();
        if result.1.timed_out() {
            return -1;
        }
        0
    }

    pub fn await_forever(&self) {
        let lock = self.m_count.lock().unwrap();
        let _ = self.m_cond.wait(lock);
    }
}