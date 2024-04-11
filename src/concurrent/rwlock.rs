use std::ops::{Add, Sub};
use std::thread::ThreadId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

use super::condition::TarCondition;
use super::mutex::TarMutex;
use crate::lang::system;
use crate::Autolock;

pub struct TarWrLock {
    m_core:Arc<Mutex<TarRwLockCore>>,
    m_rd_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_rd_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,

    m_wr_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_wr_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>
}

impl TarWrLock {
    fn new(core:Arc<Mutex<TarRwLockCore>>,
        rd_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
        rd_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,
        wr_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
        wr_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>)->Self {
        
        TarWrLock {
            m_core:core,
            m_rd_tx:rd_tx,
            m_rd_rx:rd_rx,
            m_wr_tx:wr_tx,
            m_wr_rx:wr_rx
        }
    }

    pub fn lock(&self) {
        let tid = system::my_tid();
        let mut guard = self.m_core.lock().unwrap();
        if let Some(threadid) = guard.m_wr_owner {
            if threadid == tid {
                guard.m_wr_owner_count += 1;
                return
            }
        }

        loop {
            if !guard.m_is_write {
                let result = guard.m_read_owners.get(&tid);
                match result {
                    Some(count) => {
                        if *count != 0 && guard.m_read_owners.len() == 1 {
                            guard.m_wr_owner_count += 1;
                            break;
                        }
                    },
                    None => {}
                }
            }

            guard.m_write_req_count += 1;
            while !guard.m_read_owners.is_empty() || guard.m_is_write {
                drop(guard);
                let lock = self.m_wr_rx.lock().unwrap();
                let _ = lock.recv();
                guard = self.m_core.lock().unwrap();
            }

            guard.m_write_req_count -= 1;
            guard.m_wr_owner_count += 1;
            break;
        }

        guard.m_is_write = true;
        guard.m_wr_owner = Some(tid);
    }

    pub fn unlock(&self) {
        let tid = system::my_tid();
        let mut guard = self.m_core.lock().unwrap();

        match guard.m_wr_owner {
            Some(threadid)=> {
                if threadid != tid {
                    panic!("wrong write lock owner");
                }
            },
            None => {
                panic!("no write lock owner");
            }
        }

        guard.m_wr_owner_count -= 1;

        if guard.m_wr_owner_count == 0 {
            guard.m_is_write = false;
            guard.m_wr_owner = None;
            if guard.m_write_req_count == 0 {
                for _ in 0..guard.m_read_wait_count {
                    let _ =  self.m_rd_tx.send(1);
                }
            } else {
                let _ = self.m_wr_tx.send(1);
            }
        }
    }


}


pub struct TarRdLock {
    m_core:Arc<Mutex<TarRwLockCore>>,
    m_rd_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_rd_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,

    m_wr_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_wr_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,
}

impl TarRdLock {
    fn new(core:Arc<Mutex<TarRwLockCore>>,
        rd_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
        rd_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,
        wr_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
        wr_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>)->Self {
        
        TarRdLock {
            m_core:core,
            m_rd_tx:rd_tx,
            m_rd_rx:rd_rx,
            m_wr_tx:wr_tx,
            m_wr_rx:wr_rx
        }
    }

    pub fn lock(&self) {
        let tid = system::my_tid();
        let mut guard = self.m_core.lock().unwrap();
        loop {
            if let Some(owner_tid) = guard.m_wr_owner {
                if owner_tid != tid && guard.m_wr_owner_count != 0 {
                    guard.m_read_wait_count += 1;
                    drop(guard);
                    //self.m_read_condition.wait(self.m_mutex.clone());
                    let mut rd_lock = self.m_rd_rx.lock().unwrap();
                    let _ =  rd_lock.recv();
                    guard = self.m_core.lock().unwrap();
                    guard.m_read_wait_count -= 1;
                    continue;
                }
            }
            break;
        }

        let result = guard.m_read_owners.get_mut(&tid);
        match result {
            Some(count) =>{
                let _ = count.add(1);
            },
            None => {
                guard.m_read_owners.insert(tid, 1);
            }
        }
    }

    pub fn unlock(&self) {
        let tid = system::my_tid();
        let mut guard = self.m_core.lock().unwrap();

        let result = guard.m_read_owners.get_mut(&tid);
        match result {
            Some(count) =>{
                if *count == 0 {
                    panic!("read lock count is zero!!");
                }
                *count -= 1;
                if *count == 0  {
                    guard.m_read_owners.remove(&tid);
                }
            },
            None => {
                panic!("not read lock");
            }
        }

        if guard.m_read_owners.len() == 0 && guard.m_write_req_count > 0 {
            self.m_wr_tx.send(1);
        }
    }
}


struct TarRwLockCore {
    m_write_req_count:u32,
    m_read_wait_count:u32,
    m_is_write:bool,
    m_wr_owner:Option<ThreadId>,
    m_wr_owner_count:u32,
    m_read_owners:HashMap<ThreadId,u32>,
}

impl TarRwLockCore {
    fn new()->Self {
        TarRwLockCore {
            m_write_req_count:0,
            m_read_wait_count:0,
            m_is_write:false,
            m_wr_owner:None,
            m_wr_owner_count:0,
            m_read_owners:HashMap::new(),
        }
    }
}
pub struct TarRwLock {
    m_core:Arc<Mutex<TarRwLockCore>>,
    m_rd_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_rd_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,

    m_wr_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_wr_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,
}

impl TarRwLock {    
    pub fn new()->Self {
        let (rd_tx,rd_rx) = mpsc::sync_channel::<u32>(256);
        let (wr_tx,wr_rx) = mpsc::sync_channel::<u32>(256);

        TarRwLock {
            m_core:Arc::new(Mutex::new(TarRwLockCore::new())),
            m_rd_tx : Arc::new(rd_tx),
            m_rd_rx : Arc::new(Mutex::new(rd_rx)),
            m_wr_tx : Arc::new(wr_tx),
            m_wr_rx : Arc::new(Mutex::new(wr_rx)),
        }
    }

    pub fn acquire_rd_lock(&self)->TarRdLock {
        TarRdLock::new(self.m_core.clone(),
                       self.m_rd_tx.clone(), 
                       self.m_rd_rx.clone(), 
                       self.m_wr_tx.clone(), 
                       self.m_wr_rx.clone())
    }

    pub fn acquire_wr_lock(&self)->TarWrLock {
        TarWrLock::new(self.m_core.clone(),
                       self.m_rd_tx.clone(), 
                       self.m_rd_rx.clone(), 
                       self.m_wr_tx.clone(), 
                       self.m_wr_rx.clone())
    }
    
}
