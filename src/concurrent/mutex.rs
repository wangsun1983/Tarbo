use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::sync::MutexGuard;
use std::thread::ThreadId;
use std::time::Duration;

struct TarMutexRecord {
    m_count:u32,
    m_ownerid:Option<ThreadId>,
}

pub struct TarMutex {
    m_record:Mutex<TarMutexRecord>,
    m_cond:Condvar,
}

impl TarMutex {
    pub fn new()->Self {
        TarMutex {
            m_record:Mutex::new(TarMutexRecord {
                m_count:0,
                m_ownerid:None,
            }),
            m_cond:Condvar::new(),
        }
    }
    
    pub fn lock(&self) {
        let tid = std::thread::current().id();
        loop {
            let mut lock = self.m_record.lock().unwrap();
            if lock.m_count == 0 {
                //no owner
                lock.m_count += 1;
                lock.m_ownerid = Some(tid);
                return;
            } else if let Some(ownerid) = lock.m_ownerid{
                if ownerid == tid {
                    lock.m_count += 1;
                    return;
                }
            }
            let _ = self.m_cond.wait(lock);
        }
    }

    pub fn lock_timeout(&self,interval:u64)->i32 {
        let tid = std::thread::current().id();
        loop {
            let mut lock = self.m_record.lock().unwrap();
            if lock.m_count == 0 {
                //no owner
                lock.m_count += 1;
                lock.m_ownerid = Some(tid);
                return 0;
            } else if let Some(ownerid) = lock.m_ownerid{
                if ownerid == tid {
                    lock.m_count += 1;
                    return 0;
                }
            }

            let result = self.m_cond.wait_timeout(lock,Duration::from_millis(interval)).unwrap();
            if result.1.timed_out() {
                return -1;
            }
        }
    }

    pub fn unlock(&self) {
        let tid = std::thread::current().id();
        let mut lock = self.m_record.lock().unwrap();
        if let Some(ownerid) = lock.m_ownerid{
            if ownerid == tid {
                lock.m_count -= 1;
                if lock.m_count == 0 {
                    lock.m_ownerid = None;
                    self.m_cond.notify_all();
                }
            }
        }
    }

    pub fn is_owner(&self)->bool {
        let tid = std::thread::current().id();
        let lock = self.m_record.lock().unwrap();
        if let Some(ownerid) = lock.m_ownerid{
            if ownerid == tid {
                return true;
            }
        }

        false
    }
}

pub struct TarAutoMutex {
    m_mutex:Arc<TarMutex>
}

impl TarAutoMutex {
    pub fn new(m:Arc<TarMutex>)->Self {
        m.lock();
        TarAutoMutex {
            m_mutex:m
        }
    }
}

impl Drop for TarAutoMutex {
    fn drop(&mut self) {
        self.m_mutex.unlock();
    }
}

#[macro_export]
macro_rules! TarAutolock {
    ($x:ident,$y:expr) => {
        let $x = crate::concurrent::mutex::TarAutoMutex::new($y.clone());
    }
}