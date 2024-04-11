use crate::lang::system;

pub struct TarTimeWatcher {
    rec:u64
}

impl TarTimeWatcher {
    pub fn new()->Self {
        TarTimeWatcher {
            rec:system::current_millis()
        }
    }

    pub fn start(&mut self) {
        self.rec = system::current_millis();
    }

    pub fn stop(&self) ->u64 {
        system::current_millis() - self.rec
    }
}

pub struct TarAutoTimeWatcher {
    rec:u64,
    tag:String
}

impl TarAutoTimeWatcher {
    pub fn new(tag:&str)->Self {
        TarAutoTimeWatcher {
            rec:system::current_millis(),
            tag:String::from(tag)
        }
    }

}

impl Drop for TarAutoTimeWatcher {
    fn drop(&mut self) {
        println!("{} cost {}",self.tag,system::current_millis() - self.rec);
    }
}