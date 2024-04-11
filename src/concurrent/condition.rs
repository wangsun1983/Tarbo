use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use super::mutex::TarMutex;

pub struct TarCondition {
    //m_cond:Condvar,
    m_tx:std::sync::mpsc::SyncSender<u32>,
    m_rx:Mutex<std::sync::mpsc::Receiver<u32>>,
    m_wait_count:Mutex<u32>
}

impl TarCondition {
    pub fn new()->Self {
        let (tx,rx) = mpsc::sync_channel::<u32>(256);
        TarCondition {
            //m_cond:Condvar::new(),
            m_tx:tx,
            m_rx:Mutex::new(rx),
            m_wait_count:Mutex::new(0)
        }
    }

    pub fn notify_all(&self) {
        let mut count = self.m_wait_count.lock().unwrap();
        for _ in 0..*count {
            let _ = self.m_tx.send(1);
        }
        *count = 0;
    }

    pub fn notify(&self) {
        let count = self.m_wait_count.lock().unwrap();
        if *count > 0 {
            let _ = self.m_tx.send(1);
        }
    }

    pub fn wait(&self,m:Arc<TarMutex>) {
        {
            let mut count = self.m_wait_count.lock().unwrap();
            *count += 1;
        }

        if !m.is_owner() {
            panic!("did not lock before condition wait");
        }

        m.unlock();
        let rx = self.m_rx.lock().unwrap();
        let _ = rx.recv();
        m.lock();
    }
}