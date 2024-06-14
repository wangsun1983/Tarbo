use std::sync::Mutex;
use std::sync::Condvar;
use std::collections::LinkedList;

//---- TarBlockingQueue ----
pub struct TarBlockingQueue<T> {
    mutex:Mutex<LinkedList<T>>,
    cond:Condvar,
}

impl <T> TarBlockingQueue<T> {
    pub fn new()->Self {
        TarBlockingQueue {
            mutex:Mutex::new(LinkedList::new()),
            cond:Condvar::new(),
        }
    }

    pub fn takeFirst(&self)-> T {
        loop {
            let mut result = self.mutex.lock().unwrap();
            if result.len() == 0 {
                self.cond.wait(result);
                continue;
            }

            return result.pop_front().unwrap();
        }
    }

    pub fn takeLast(&self)-> T {
        loop {
            let mut result = self.mutex.lock().unwrap();
            if result.len() == 0 {
                self.cond.wait(result);
                continue;
            }

            return result.pop_back().unwrap();
        }
    }

    pub fn putFront(&self,val:T) {
        let mut result = self.mutex.lock().unwrap();
        result.push_front(val);
        self.cond.notify_one();
    }

    pub fn putLast(&self,val:T) {
        let mut result = self.mutex.lock().unwrap();
        result.push_back(val);
        self.cond.notify_one();
    }
}

//--- TarConcurrentQueue ---
pub struct TarConcurrentQueue<T> {
    mutex:Mutex<LinkedList<T>>,
}

impl <T> TarConcurrentQueue<T> {
    pub fn new()->Self {
        TarConcurrentQueue {
            mutex:Mutex::new(LinkedList::new()),
        }
    }

    pub fn putFirst(&self,data:T) {
        let mut list = self.mutex.lock().unwrap();
        list.push_front(data);
    }

    pub fn putLast(&self,data:T) {
        let mut list = self.mutex.lock().unwrap();
        list.push_back(data);
    }

    pub fn takeLast(&self) -> Option<T> {
        let mut list = self.mutex.lock().unwrap();
        list.pop_back()
    }

    pub fn takeFirst(&self) -> Option<T> {
        let mut list = self.mutex.lock().unwrap();
        list.pop_front()
    }
}

//thread local
