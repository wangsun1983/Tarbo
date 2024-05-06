use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cell::Cell;
use std::ptr::NonNull;

use std::rc::Rc;
use std::sync::Arc;
use std::sync::{Mutex,Condvar};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use tokio::time::Interval;

use crate::lang::system;
use std::sync::atomic::AtomicU32;


// pub trait TarProcessMessage {
//     fn handleMessage(&self,msg:TarMessage);
// }
type TarProcessMessage = fn(msg:TarMessage);

//---- Message ----
pub struct TarMessage {
    pub what:u32,
    pub arg1:i32,
    pub arg2:i32,
    pub next_time:u64,
    pub next:Option<Rc<RefCell<TarMessage>>>,
    pub target:Option<Arc<Box<TarProcessMessage>>>,
    pub handle_id:u32
}

impl TarMessage {
    pub fn new(what:u32)->Self {
        TarMessage {
            what:what,
            arg1:0,
            arg2:0,
            next_time:0,
            next:None,
            target:None,
            handle_id:0
        }
    }

    fn set_next(&mut self,next:TarMessage)->&mut Self {
        self.next = Some(Rc::new(RefCell::new(next)));//Some(RefCell::new(Rc::new(Box::new(next))));
        self
    }

    // pub fn set_target(&mut self,target:Box<dyn ProcessMessage>)->&mut Self {
    //     self.target = Some(target);
    //     self
    // }

    fn execute(&self) {
        match &self.target {
            None=> {},
            Some(handler) => {
                handler(
                TarMessage{
                        what:self.what,
                        arg1:self.arg1,
                        arg2:self.arg2,
                        next_time:self.next_time,
                        next:None,
                        target:None,
                        handle_id:0
                    }
                )
            }
        }
    }
}

//---- MessageQueue ----
pub struct TarMessageQueue {
    head:Mutex<TarMessage>,
    cond:Condvar,
    is_running:AtomicU32,
}

impl TarMessageQueue {
    pub fn new()->Self {
        TarMessageQueue {
            head:Mutex::new(TarMessage::new(0)),
            cond:Condvar::new(),
            is_running:AtomicU32::new(0)
        }
    }

    pub fn remove_message_by_id(&self,handle_id:u32) {
        self.remove_message(false,0,handle_id);
    }

    pub fn remove_message_by_what_id(&self,what:u32,handle_id:u32) {
        self.remove_message(true,what,handle_id);
    }

    pub fn remove_message(&self,is_need_check_what:bool,what:u32,handle_id:u32) {
        let mut head = self.head.lock().unwrap();
        if let Some(v) = &head.next {
            let mut prev = v.clone();
            let mut current = v.clone();
            let mut pos:u32 = 0;
            println!("remove message trace2,what is {},handle_id is {}",what,handle_id);
            loop {
                let w = current.try_borrow_mut().unwrap().what;
                let id = current.try_borrow_mut().unwrap().handle_id;
                println!("current is what is {},id is {}",w,id);
                let mut what_check = true;
                if is_need_check_what {
                    what_check = w == what;
                }

                if what_check && 
                   id == handle_id {
                    if pos == 0 {
                        head.next = current.try_borrow().unwrap().next.clone();

                        match head.next.clone() {
                            None=> {
                                return;
                            },
                            Some(next_node)=> {
                                current = next_node.clone();
                            }
                        }
                        continue;
                    } else {
                        pos += 1;
                        let next = current.try_borrow().unwrap().next.clone();
                        prev.try_borrow_mut().unwrap().next = next.clone();
                        match next.clone() {
                            None=> {
                                return;
                            },
                            Some(next_node)=> {
                                current = next_node.clone();
                            }
                        }
                    }
                } else {
                    pos += 1;
                    prev = current.clone();
                    let next = current.clone().try_borrow().unwrap().next.clone();
                    match next {
                        None=> {return;},
                        Some(next_node) =>{
                            current = next_node.clone();
                        },
                    }
                }
            }
        }
    }

    pub fn dequeue_message(&self)->Option<Rc<RefCell<TarMessage>>> {
        println!("dequeue_message start");
        loop {
            if self.is_running.fetch_add(0, std::sync::atomic::Ordering::Acquire) != 0 {
                return None;
            }

            let mut head = self.head.lock().unwrap();
            println!("dequeue_message trace1");
            match head.next.clone() {
                None => {
                    println!("dequeue_message trace2");
                    self.cond.wait(head);
                    continue;
                },
                Some(msg)=>{
                    
                    let next_time = msg.try_borrow().unwrap().next_time;
                    println!("msg next time is {},current is {}", next_time,system::current_millis());
                    if next_time <=  system::current_millis() {
                        head.next = msg.try_borrow().unwrap().next.clone();
                        return Some(msg.clone());
                    }

                    self.cond.wait_timeout(head,
                        Duration::from_millis(next_time - system::current_millis()));                   
                    
                }
            }
        }
    }

    pub fn enqueue_message(&self,mut msg:TarMessage) {
        let mut head = self.head.lock().unwrap();
        println!("enqueue message trace1", );
        if let Some(v) = &head.next {
            let mut prev = v.clone();
            let mut current = v.clone();
            let mut pos:u32 = 0;
            println!("enqueue message trace2", );
            loop {
                if current.try_borrow().unwrap().next_time > msg.next_time {
                    println!("enqueue message trace3", );
                    if pos == 0 {
                        println!("enqueue message trace4", );
                        msg.next = Some(current);
                        head.next = Some(Rc::new(RefCell::new(msg)));
                    } else {
                        println!("enqueue message trace5", );
                        msg.next = Some(current);
                        prev.try_borrow_mut().unwrap().next = Some(Rc::new(RefCell::new(msg)));
                    }
                    self.cond.notify_one();
                    return;
                } else {
                    println!("enqueue message trace6", );
                    pos += 1;
                    let mut is_none = false;
                    match current.try_borrow().unwrap().next {
                        None => {
                            is_none = true;
                        },
                        Some(_) => {}
                    }

                    if is_none {
                        current.try_borrow_mut().unwrap().next =  Some(Rc::new(RefCell::new(msg)));
                        self.cond.notify_one();
                        return;
                    }
                    prev = current.clone();
                    let tmp: Rc<RefCell<TarMessage>> = current.try_borrow_mut().unwrap().next.clone().unwrap().clone();
                    current = tmp;
                }
            }
        } else {
            println!("enqueue message trace8", );
            head.next = Some(Rc::new(RefCell::new(msg)));
            self.cond.notify_one();
            return
        }
    }

    pub fn quit(&self) {
        self.is_running.fetch_add(1, std::sync::atomic::Ordering::Acquire);
    }
}

//dff
pub struct Looper {
    queue:TarMessageQueue,
    id:Mutex<u32>,
}

impl Looper {
    fn new()->Self {
        Looper {
            queue:TarMessageQueue::new(),
            id:Mutex::new(0),
        }
    }

    fn generate_id(&self)->u32 {
        let mut value = self.id.lock().unwrap();
        *value += 1;
        *value
    }

    fn loop_self(&self) {
        loop {
            let msg = self.queue.dequeue_message();
            match msg {
                None=>{
                    return;
                },
                Some(m) => {
                    let _msg = m.clone();
                    _msg.try_borrow_mut().unwrap().execute();
                }
            }
        }
    }
    
    fn enqueue(&self,msg:TarMessage) {
        self.queue.enqueue_message(msg);
    }

    fn remove(&self,what:u32,handle_id:u32) {
        self.queue.remove_message_by_what_id(what, handle_id)
    }

    fn remove_by_id(&self,handle_id:u32) {
        self.queue.remove_message_by_id(handle_id)
    }

    fn quit(&self) {
        self.queue.quit();
    }
}
pub struct TarHandlerThread {
    looper:Arc<Looper>,
    //join_handler:Option<JoinHandle<()>>,
    join_handlers:Vec<JoinHandle<()>>,
}


struct InternalLooper {
    looper:Arc<Looper>
}

unsafe impl Send for InternalLooper{}

impl TarHandlerThread {
    pub fn new()->Self {
        TarHandlerThread {
            looper:Arc::new(Looper::new()),
            //join_handler:None,
            join_handlers:Vec::new()
        }
    }

    pub fn start(&mut self) {
        let internal_looper: Arc<Mutex<InternalLooper>> = Arc::new(Mutex::new(InternalLooper {
            looper:self.looper.clone()
        }));

        let closure_loop = internal_looper.clone();
        let join_handler = thread::spawn(move||{
            println!("closure loop start!!!");
            let ll= closure_loop.lock().unwrap();
            ll.looper.loop_self();
        });

        self.join_handlers.push(join_handler);
    }

    pub fn get_looper(&self)->Arc<Looper> {
        self.looper.clone()
    }

    pub fn quit(&self) {
        self.looper.quit();
    }

    pub fn wait_termination(&mut self) {
        while self.join_handlers.len() > 0 {
            let h = self.join_handlers.pop();
            match h {
                None =>{},
                Some(handle)=> {
                    handle.join();
                }
            }
        }
    }
    
}

pub struct TarHandler {
    looper:Arc<Looper>,
    processor:Arc<Box<TarProcessMessage>>,
    self_thread:Option<TarHandlerThread>,
    id:u32
}

impl TarHandler {
    pub fn new(processor:Box<TarProcessMessage>)->Self {
        let mut handler_th: TarHandlerThread = TarHandlerThread::new();
        handler_th.start();
        
        TarHandler {
            looper:handler_th.get_looper().clone(),
            processor:Arc::new(processor),
            id:handler_th.get_looper().generate_id(),
            self_thread:Some(handler_th)
        }
    }

    pub fn new_with_looper(processor:Box<TarProcessMessage>,looper:Arc<Looper>)->Self {
        TarHandler {
            looper:looper.clone(),
            processor:Arc::new(processor),
            self_thread:None,
            id:looper.clone().generate_id()
        }
    }
    
    pub fn send_empty_message(&self,what:u32) {
        println!("send_empty_message start");
        let mut msg = TarMessage::new(what);
        msg.target = Some(self.processor.clone());
        msg.handle_id = self.id;
        self.looper.enqueue(msg);
    }

    pub fn send_empty_message_delayed(&self,what:u32,interval:u64) {
        println!("send_empty_message_delayed start");
        let mut msg = TarMessage::new(what);
        msg.next_time = system::current_millis() + interval;
        msg.target = Some(self.processor.clone());
        msg.handle_id = self.id;
        self.looper.enqueue(msg);
    }

    pub fn quit(&self) {
        match &self.self_thread {
            None=> {
                self.looper.remove_by_id(self.id);
            },
            Some(thread)=> {
                thread.quit();
            }
        }
    }

    pub fn remove_messages(&self,what:u32) {
        self.looper.remove(what,self.id);
    }
}