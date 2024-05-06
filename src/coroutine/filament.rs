use tokio::io;
use tokio::sync::mpsc;
use std::future::Future;
use std::sync::Arc;
use std::cell::RefCell;
use std::thread;
use futures::task;


use futures::executor::block_on;
use tokio::task::JoinHandle;
pub struct Filament {
    runtime:tokio::runtime::Runtime,
}

pub struct FilamentTask {
    func:Option<Box<dyn Fn() + 'static + Send>>,
    run:Option<Box<dyn FilamentRunnable + Sync +Send>>
}

impl FilamentTask {
    fn new_by_fn(func:Box<dyn Fn() + 'static + Send>)->Self {
        FilamentTask {
            func:Some(func),
            run:None
        }
    }
}

pub trait FilamentRunnable {
    fn run(&self);
}

impl Filament {
    pub fn new()->Self {
        let runtime = tokio::runtime::Builder::new_multi_thread().max_threads(4).enable_all().build().unwrap();
        Filament {
            runtime:runtime,
        }
    }

    pub fn wait_closure<F,T>(&self,func:F)->T where
                                F:Fn()->T + Send + 'static {
        self.runtime.block_on(async move {
            func()
        })
    }

    pub fn wait_future<F,T>(&self,func:F)->T where F:Future<Output = T>,T:Send {
        self.runtime.block_on(async {
            (func).await
        })
    }

    pub fn execute(&self,run:Box<dyn FilamentRunnable + Sync +Send>) {
        self.runtime.spawn(async move {
            run.run();
        });
    }


    pub fn execute_closure<F>(&self,func:F) where F:Fn() + Send + 'static {
        self.runtime.spawn(async move {
            func();
        });
    }

    pub fn execute_future<F>(&self,func:F) where F: Future + Send + 'static,
                                                 F::Output: Send + 'static {
        self.runtime.spawn(async {
            (func).await
        });
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output> where
            F: Future + Send + 'static,
            F::Output: Send + 'static {
            self.runtime.spawn(future)
    }

}