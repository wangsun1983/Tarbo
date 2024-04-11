use crate::concurrent::container::TarBlockingQueue;
use std::process::Output;
use std::thread;
use std::sync::{Arc, Condvar};
use std::sync::Mutex;

pub trait TarExecutor {
    fn submit<T>(&mut self,task:T) where T:FnOnce() + Send + 'static;
    fn shutdown(&mut self);
    fn awaitTermination(&mut self);
}

pub struct TarFuture<T> {
    mutex:Arc<Mutex<(u32,T)>>,
    cond:Condvar
}

struct TarExecutorTask {
    func:Box<dyn FnOnce() + 'static + Send>,
}

//---- ThreadPoolExecutor ----
pub struct TarThreadPoolExecutor {
    queue:Arc<TarBlockingQueue<Option<TarExecutorTask>>>,
    handlers:Option<Vec<Option<thread::JoinHandle<()>>>>
}

impl TarThreadPoolExecutor {
    pub fn new(size:u32)->Self {
        let queue = Arc::new(TarBlockingQueue::<Option<TarExecutorTask>>::new());
        let mut handlers:Vec<Option<thread::JoinHandle<()>>> = Vec::new();

        for _ in 0..size {
            let tasks = queue.clone();
            let handler = thread::spawn(move|| {
                let task: Option<TarExecutorTask> = tasks.takeFirst();
                match task {
                    Some(fun) => {
                        (fun.func)();
                    },
                    None => {
                        tasks.putFront(None);
                        return;
                    }
                }
                
            });
            handlers.push(Some(handler));
        }

        TarThreadPoolExecutor {
            queue:queue.clone(),
            handlers:Some(handlers)
        }
    }
}

impl TarExecutor for TarThreadPoolExecutor {
    fn submit<T>(&mut self,task:T) where T:FnOnce() + Send + 'static {
        self.queue.putLast(Some(TarExecutorTask{
            func:Box::new(task)
        }));
    }

    fn shutdown(&mut self) {
        self.queue.putFront(None);
    }

    fn awaitTermination(&mut self) {
        match self.handlers {
            Some(_) => {
                for handler in self.handlers.take().unwrap() {
                    match handler {
                        Some(h) => {
                            h.join();
                        }
                        None => {
                            //do nothing
                        }
                    }
                }
            },
            None => {}
        }
    }
}