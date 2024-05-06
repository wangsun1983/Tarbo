use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedWriteHalf};


use super::address::TarAddress;
use super::socket::TarSocket;

use crate::coroutine::filament;

pub enum Event {
    Connect,
    Message,
    Disconnect,
    NewClient,
}

pub trait TarListener {
    fn on_event(&self,event:Event,data:Option<Vec<u8>>,sock:Arc<TarSocket>);
}

pub struct TarSocketMonitor {
    fila:Arc<filament::Filament>
}

impl TarSocketMonitor {
    pub fn new()->Self {
        TarSocketMonitor {
            fila:Arc::new(filament::Filament::new())
        }

    }

    pub fn bind_as_client(&self,sock:Arc<TarSocket>,listener:Box<dyn TarListener + Send + Sync>) {
        self.fila.spawn(async move {
            let input_stream = sock.get_input_stream();
            
            loop {
                let read_result = input_stream.read_async().await;
                match read_result {
                    Some(data)=> {
                        listener.on_event(Event::Message,Some(data),sock.clone());
                    },
                    None => {
                        listener.on_event(Event::Disconnect,None,sock.clone());
                        return;
                    }
                }
            }
        });
    }

    pub fn bind_as_server(&self,sock:Arc<TarSocket>,listener:Box<dyn TarListener+Send+Sync>) {
        let fila_closure = self.fila.clone();
        self.fila.clone().spawn(async move {
            let accept_result = sock.accept_async().await;
            match accept_result {
                Some(tcp_stream) => {
                    let client = Arc::new(tcp_stream);
                    listener.on_event(Event::NewClient, None, client.clone());
                    let input_stream = client.get_input_stream();
                    fila_closure.spawn(async move {
                        loop {
                            let read_result = input_stream.read_async().await;
                            match read_result {
                                Some(data)=> {
                                    listener.on_event(Event::Message,Some(data),client.clone());
                                },
                                None => {
                                    listener.on_event(Event::Disconnect,None,client.clone());
                                    return;
                                }
                            }
                        }
                    });
                },
                None=> {
                    return;
                }
            }
        });
    }
}