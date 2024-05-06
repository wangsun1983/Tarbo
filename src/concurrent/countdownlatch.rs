use std::sync::Mutex;
use std::sync::Condvar;
use std::time::Duration;

pub struct TarCountDownLatch {
    m_count:Mutex<usize>,
    m_cond:Condvar
}

impl TarCountDownLatch {
    pub fn new(count:usize)->Self {
        TarCountDownLatch {
            m_count:Mutex::new(count),
            m_cond:Condvar::new()
        }
    }

    pub fn count_down(&self) {
        let mut lock = self.m_count.lock().unwrap();

        if *lock > 0 {
            *lock -= 1;
            if *lock == 0 {
                self.m_cond.notify_all();
            }
        }
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
        if *lock != 0 {
            let _ = self.m_cond.wait(lock);
        }
    }
}