use std::time::Duration;
use std::sync::Arc;

use crate::coroutine::filament::{self, Filament};

pub fn testFilament_1() {
    let fila = Arc::new(Filament::new());
    let closure = fila.clone();
    
    let v = fila.clone().wait_closure(move ||->String {
        println!("sync closure!!!!", );
        String::from("hello")
    });

    println!("v is {}",v);

    std::thread::sleep(Duration::from_secs(100));
}