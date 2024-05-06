
use std::sync::mpsc;
use std::sync::Condvar;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;

use std::thread;
use std::time::Duration;
use std::collections::LinkedList;

use crate::concurrent::container;
use crate::lang::system;

use super::countdownlatch::TarCountDownLatch;

struct TarExecutorTaskCore<T> where T:Send+Sync {
    m_complete:bool,
    m_result:Arc<Option<T>>,
    m_tx:Arc<std::sync::mpsc::SyncSender<u32>>,
    m_rx:Arc<Mutex<std::sync::mpsc::Receiver<u32>>>,
    m_wait_count:u32
}

impl <T>TarExecutorTaskCore<T> where T:Send+Sync {
    fn new(tx:std::sync::mpsc::SyncSender<u32>,
           rx:std::sync::mpsc::Receiver<u32>)->Self {
        TarExecutorTaskCore {
            m_complete:false,
            m_result:Arc::new(None),
            m_tx:Arc::new(tx),
            m_rx:Arc::new(Mutex::new(rx)),
            m_wait_count:0
        }
    }

    fn complete(&mut self) {
        self.m_complete = true;
    }

    fn notify(&mut self) {
        if self.m_wait_count != 0 {
            self.m_tx.send(1);
            self.m_wait_count -= 1;
        }
    }

    fn notify_all(&mut self) {
        for _ in 0..self.m_wait_count {
            self.m_tx.send(1);
        }
        self.m_wait_count = 0;
    }

    fn wait(&mut self) {
        self.m_wait_count += 1;
        self.m_rx.lock().unwrap().recv();
    }
}

pub struct TarExecutorTask<T> where T:Send + Sync + 'static {
    core:Arc<Mutex<TarExecutorTaskCore<T>>>,
    m_fun:Box<dyn Fn()->T + 'static + Send>,
}

impl <T>TarExecutorTask<T> where T:Send+ Sync + 'static{
    fn new<F>(func:F)->Self where F:Fn()->T + Send + 'static {
        let (tx,rx) = mpsc::sync_channel::<u32>(1);
        let core = Arc::new(Mutex::new(TarExecutorTaskCore::new(tx,rx)));
        TarExecutorTask {
            core:core.clone(),
            m_fun:Box::new(func)
        }
    }

    pub fn execute(&self) {
        let t_result = (self.m_fun)();
        let mut task = self.core.lock().unwrap();        
        if task.m_complete {
            return;
        }
        task.m_result = Arc::new(Some(t_result));
        task.m_complete = true;
        //task.m_tx.send(1);
        task.notify();
    }

    fn get_core(&self)->Arc<Mutex<TarExecutorTaskCore<T>>> {
        self.core.clone()
    }
}

pub struct TarFuture<T> where T:Send + Sync + 'static {
    core:Arc<Mutex<TarExecutorTaskCore<T>>>,
}

impl <T>TarFuture<T> where T:Send + Sync + 'static {
    fn new(core:Arc<Mutex<TarExecutorTaskCore<T>>>)->Self {
        TarFuture {
            core:core.clone()
        }
    }

    pub fn get(&self)->Arc<Option<T>> {
        let core_closure = self.core.clone();
        let mut core = core_closure.lock().unwrap();
        if !core.m_complete {
            let rx_closure = core.m_rx.clone();
            let lock = rx_closure.lock().unwrap();
            core.m_wait_count += 1;
            drop(core);
            let _ = lock.recv().unwrap();
            core = core_closure.lock().unwrap();
        }
        core.m_result.clone()
    }
    

    pub fn wait(&self) {
        self.local_wait(0);
    }

    pub fn wait_timeout(&self,interval:u64)->i32 {
        self.local_wait(interval)
    }

    fn local_wait(&self,interval:u64)->i32 {
        let core_closure = self.core.clone();
        let mut core = core_closure.lock().unwrap();
        if !core.m_complete {
            let rx_closure = core.m_rx.clone();
            let lock = rx_closure.lock().unwrap();
            drop(core);

            if interval == 0 {
                let _ = lock.recv().unwrap();
            } else {
                let ret = lock.recv_timeout(Duration::from_millis(interval));
                match ret {
                    Ok(_)=>{},
                    Err(_)=>{
                        return -1;
                    }
                }
            }

            core = core_closure.lock().unwrap();
            core.m_complete = true;
        }
        0
    }
}

//--------------- TarPoolExecutor ---------------//
pub struct TarPoolExecutor<T> where T:Send + Sync + 'static{
    m_queue:Arc<container::TarBlockingQueue<Option<TarExecutorTask<T>>>>,
    m_latch:Arc<TarCountDownLatch>,
}

impl <T>TarPoolExecutor<T> where T:Send + Sync + 'static{
    pub fn new(size:usize)->Self {
        let latch = Arc::new(TarCountDownLatch::new(size));
        let queue = Arc::new(container::TarBlockingQueue::new());
        
        let mut executor = TarPoolExecutor {
            m_latch:latch.clone(),
            m_queue:queue.clone(),
        };

        for _ in 0..size {
            let latch_closure = latch.clone();
            let queue_closure = queue.clone();

            std::thread::spawn(move || {
                loop {
                    let task = queue_closure.takeFirst();
                    match task {
                        Some(t) => {
                            t.execute();
                        },
                        None => {
                            println!("thread pool exit!!!!");
                            queue_closure.putFront(None);
                            break;
                        }
                    }
                }

                latch_closure.count_down();
            });
        }

        executor
    }

    pub fn submit<F>(&self,task:F)->TarFuture<T> where F:Fn()->T + Send + 'static {
        let task = TarExecutorTask::new(task);
        let future = TarFuture::new(task.core.clone());
        self.m_queue.putLast(Some(task));
        future
    }

    pub fn shut_down(&self) {
        self.m_queue.putFront(None);
    }

    pub fn wait_termination(&self) {
        self.m_latch.clone().await_forever();
    }
}

//--------------- TarSchedulePoolExecutor ---------------//

pub struct TarScheduleTask<T> where T:Send + Sync + 'static {
    m_core:Option<Arc<Mutex<TarExecutorTaskCore<T>>>>,
    m_func:Option<Arc<Box<dyn Fn()->T + Send + 'static>>>,
    m_next_time:u64,
    m_next:Option<Arc<RefCell<TarScheduleTask<T>>>>,
}

impl <T> TarScheduleTask<T> where T:Send + Sync + 'static {
    fn new(func:Box<dyn Fn()->T + Send + 'static>)->Self {
        let (tx,rx) = mpsc::sync_channel::<u32>(1);
        let core = Arc::new(Mutex::new(TarExecutorTaskCore::new(tx,rx)));
        TarScheduleTask {
            m_core:Some(core.clone()),
            m_func:Some(Arc::new(func)),
            m_next_time:0,
            m_next:None
        }
    }
}

unsafe impl<T> Send for TarScheduleTask<T> where T:Send + Sync + 'static{}


pub struct TarSchedulePoolExecutor<T> where T:Send + Sync + 'static {
    m_pool_executor:Arc::<TarPoolExecutor<()>>,
    m_head:Arc<Mutex<TarScheduleTask<T>>>,
    m_cond:Arc<Condvar>,
    m_mutex:Arc<Mutex<u32>>,
    m_latch:Arc<TarCountDownLatch>
}

struct InnerTarScheduleTask<T> where T:Send + Sync + 'static {
    m_core:Arc<Mutex<TarExecutorTaskCore<T>>>,
    m_func:Arc<Box<dyn Fn()->T + Send + 'static>>,
}

unsafe impl<T> Send for InnerTarScheduleTask<T> where T:Send + Sync + 'static{}

impl <T> TarSchedulePoolExecutor<T> where T:Send + Sync + 'static {
    pub fn new(size:usize) -> Self {
        let _head = Arc::new(Mutex::new(TarScheduleTask {
            m_core:None,
            m_func:None,
            m_next_time:0,
            m_next:None
        }));

        let _pool_executor = Arc::new(TarPoolExecutor::<()>::new(size));
        let _cond = Arc::new(Condvar::new());
        let _latch = Arc::new(TarCountDownLatch::new(1));
        let _mutex = Arc::new(Mutex::new(1));
        
        let _head_closure = _head.clone();
        let _pool_closure = _pool_executor.clone();
        let _cond_closure = _cond.clone();
        let _latch_closure = _latch.clone();
        let _mutex_closure = _mutex.clone();
        thread::spawn(move||{
            loop {
                {
                    let m_lock = _mutex_closure.lock().unwrap();
                    if *m_lock == 0 {
                        _latch_closure.count_down();
                        return;
                    }
                }
                let mut head_lock = _head_closure.lock().unwrap();
                if let Some(task) = head_lock.m_next.clone() {
                    let current = system::current_millis();
                    let next_time = task.try_borrow_mut().unwrap().m_next_time;
                    if next_time <= current {
                        let core = task.try_borrow_mut().unwrap().m_core.clone();
                        let core_unwrap = core.unwrap();
                        let func_unwrap = task.try_borrow_mut().unwrap().m_func.clone().unwrap();

                        let inner_task = Arc::new(Mutex::new(InnerTarScheduleTask {
                            m_core:core_unwrap.clone(),
                            m_func:func_unwrap.clone()
                        }));
                        let closure = inner_task.clone();
                        _pool_closure.submit(move||{
                            let mut core_lock = closure.lock().unwrap();
                            let func_unwrap = core_lock.m_func.clone();
                            let result = Arc::new(Some(func_unwrap()));
                            {
                                let mut result_lock = core_lock.m_core.lock().unwrap();
                                result_lock.m_result = result;
                                result_lock.complete();
                                result_lock.notify_all();
                            }
                        });
                        head_lock.m_next = task.try_borrow_mut().unwrap().m_next.clone();
                    } else {
                        _cond_closure.clone().wait_timeout(head_lock,Duration::from_millis(next_time - current));
                    }
                } else {
                    _cond_closure.clone().wait(head_lock);
                }
            }
        });
        
        TarSchedulePoolExecutor {
            m_pool_executor:_pool_executor.clone(),
            m_head:_head.clone(),
            m_cond:_cond.clone(),
            m_mutex:_mutex.clone(),
            m_latch:_latch.clone()
        }
    }

    pub fn schedule<F>(&self,interval:u64,func:F)->TarFuture<T> where F:Fn()->T + Send + 'static {
        let mut new_task = TarScheduleTask::<T>::new(Box::new(func));
        new_task.m_next_time = system::current_millis() + interval;
        
        let future = TarFuture::new(new_task.m_core.clone().unwrap());
        let mut head_lock = self.m_head.lock().unwrap();
        if let Some(task) = head_lock.m_next.clone() {
            let mut current = task.clone();
            loop {
                let next_time = current.try_borrow().unwrap().m_next_time;
                if next_time > new_task.m_next_time {
                    new_task.m_next = Some(current.clone());
                    head_lock.m_next =  Some(Arc::new(RefCell::new(new_task)));
                    self.m_cond.notify_one();
                    return future;
                } else {
                    loop {
                        let prev = current.clone();
                        let temp = prev.try_borrow_mut().unwrap().m_next.clone();
                        match temp {
                            None => {
                                prev.try_borrow_mut().unwrap().m_next =Some(Arc::new(RefCell::new(new_task)));
                                return future;
                            },
                            Some(this_task)=> {
                                let next_time = this_task.try_borrow_mut().unwrap().m_next_time;
                                if next_time < new_task.m_next_time {
                                    new_task.m_next = Some(current.clone());
                                    prev.try_borrow_mut().unwrap().m_next = Some(Arc::new(RefCell::new(new_task)));
                                    return future;
                                } else {
                                    current = this_task.clone();
                                }       
                            }
                        }
                    }
                }
            }
        } else {
            head_lock.m_next = Some(Arc::new(RefCell::new(new_task)));
            self.m_cond.notify_one();
            return future;
        }
    }

    pub fn shut_down(&self) {
        self.m_pool_executor.shut_down();
        let mut v = self.m_mutex.lock().unwrap();
        *v = 0;
        self.m_cond.notify_all();
    }

    pub fn wait_termination(&self) {
        self.m_latch.await_forever();
        self.m_pool_executor.wait_termination();
    }
    
}

pub enum TarPriority {
    High,
    Mid,
    Low
}

pub struct TarPriorityPoolExecutor<T> where T:Send + Sync + 'static {
    m_pool_executor:Arc::<TarPoolExecutor<()>>,
    m_high_list:Arc<Mutex<LinkedList<TarExecutorTask<T>>>>,
    m_mid_list:Arc<Mutex<LinkedList<TarExecutorTask<T>>>>,
    m_low_list:Arc<Mutex<LinkedList<TarExecutorTask<T>>>>,

    m_mutex:Arc<Mutex<u32>>, //0 is finish,1 is conitnue
    m_cond:Arc<Condvar>,
    m_latch:Arc<TarCountDownLatch>,
}

impl <T> TarPriorityPoolExecutor<T> where T:Send + Sync + 'static {
    pub fn new(size:usize)->Self {
        let _pool = Arc::new(TarPoolExecutor::<()>::new(size));
        let _high_list = Arc::new(Mutex::new(LinkedList::<TarExecutorTask<T>>::new()));
        let _mid_list = Arc::new(Mutex::new(LinkedList::<TarExecutorTask<T>>::new()));
        let _low_list = Arc::new(Mutex::new(LinkedList::<TarExecutorTask<T>>::new()));
        let _mutex = Arc::new(Mutex::new(1));
        let _cond = Arc::new(Condvar::new());
        let _latch = Arc::new(TarCountDownLatch::new(size));

        for _ in 0..size {
            let high_list_closure = _high_list.clone();
            let mid_list_closure = _mid_list.clone();
            let low_list_closure = _low_list.clone();
            let mutex_closure = _mutex.clone();
            let cond_closure = _cond.clone();
            let latch_closure = _latch.clone();
            _pool.submit(move||{
                //check high first
                loop {
                    {
                        let wait_lock = mutex_closure.lock().unwrap();
                        if *wait_lock == 0 {
                            latch_closure.count_down();
                            return;
                        }
                    }

                    {
                        let mut high_lock = high_list_closure.lock().unwrap();
                        if high_lock.len() != 0 {
                            let task = high_lock.pop_front().unwrap();
                            drop(high_lock);
                            task.execute();
                            continue;
                        }
                    }

                    {
                        let mut mid_lock = mid_list_closure.lock().unwrap();
                        if mid_lock.len() != 0 {
                            let task = mid_lock.pop_front().unwrap();
                            drop(mid_lock);
                            task.execute();
                            continue;
                        }
                    }

                    {
                        let mut low_lock = low_list_closure.lock().unwrap();
                        if low_lock.len() != 0 {
                            let task = low_lock.pop_front().unwrap();
                            drop(low_lock);
                            task.execute();
                            continue;
                        }
                    }

                    let wait_lock = mutex_closure.lock().unwrap();
                    let _ = cond_closure.wait(wait_lock);
                }
            });
        }

        TarPriorityPoolExecutor {
            m_pool_executor:_pool.clone(),
            m_high_list:_high_list.clone(),
            m_mid_list:_mid_list.clone(),
            m_low_list:_low_list.clone(),
            m_mutex:_mutex.clone(),
            m_cond:_cond.clone(),
            m_latch:_latch.clone()
        }
    }

    pub fn submit<F>(&self,pri:TarPriority,func:F)->TarFuture<T> where F:Fn()->T + Send + 'static {
        let task = TarExecutorTask::new(func);
        let future = TarFuture::new(task.core.clone());

        match pri {
            TarPriority::High => {
                let mut high_lock = self.m_high_list.lock().unwrap();
                high_lock.push_back(task);
            },
            TarPriority::Mid => {
                let mut mid_lock = self.m_mid_list.lock().unwrap();
                mid_lock.push_back(task);
            },
            TarPriority::Low => {
                let mut mid_lock = self.m_low_list.lock().unwrap();
                mid_lock.push_back(task);
            },
        }

        self.m_cond.notify_one();
        return future;
    }

    pub fn shut_down(&self) {
        let mut lock = self.m_mutex.lock().unwrap();
        *lock = 0;
        self.m_cond.notify_all();
    }

    pub fn wait_termination(&self) {
        self.m_latch.await_forever();
    }
    

}

