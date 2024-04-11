use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedWriteHalf};


use super::address::TarAddress;

use crate::coroutine::filament;

pub enum Event {
    Connect,
    Message,
    Disconnect,
    NewClient,
}

pub trait TarListener {
    fn on_event(&self,event:Event,data:Option<Vec<u8>>,stream:Arc<Box<TarNetStream>>);
}

pub struct TarSocketMonitor {
    fila:Arc<filament::Filament>
}


pub struct TarNetStream {
    //stream:Arc<RefCell<TcpStream>>,
    read_stream:tokio::sync::Mutex<tokio::net::tcp::OwnedReadHalf>,
    write_stream:tokio::sync::Mutex<OwnedWriteHalf>,
    fila:Arc<filament::Filament>,
    name:String,
    client_addr:TarAddress
}

impl TarNetStream {
    fn new(rd:tokio::net::tcp::OwnedReadHalf,wr:OwnedWriteHalf,f:Arc<filament::Filament>,name:String)->Self {
        TarNetStream {
            read_stream:tokio::sync::Mutex::new(rd),
            write_stream:tokio::sync::Mutex::new(wr),
            fila:f,            
            name:name,
            client_addr:TarAddress::new("",0)
        }
    }

    fn new_with_address(rd:tokio::net::tcp::OwnedReadHalf,wr:OwnedWriteHalf,f:Arc<filament::Filament>,name:String,ip:String,port:u32)->Self {
        TarNetStream{
            read_stream:tokio::sync::Mutex::new(rd),
            write_stream:tokio::sync::Mutex::new(wr),
            fila:f,            
            name:name,
            client_addr:TarAddress::new(ip.as_str(),port)
        }
    }

    pub fn read(&self)->Option<Vec<u8>> {
        self.fila.clone().wait_future(async {
            let mut buf:[u8;1024*4] = [0;1024*4];
            let mut lock_obj = self.read_stream.lock().await;
            let result = lock_obj.read(&mut buf).await;
            match result {
                Ok(size) => {
                    return Some(Vec::from(&buf[0..size]));
                },
                Err(_) => {
                    return None;
                }
            }
        })
    }

    pub fn write(&self,buf:&[u8])->Option<usize> {
        self.fila.clone().wait_future(async {
            let mut lock_obj = self.write_stream.lock().await;
            let result = lock_obj.write(buf).await;
            match result {
                Ok(size) => {
                    return Some(size);
                },
                Err(_) => {
                    return None;
                }
            }
        })
    }

    pub fn get_address<'b>(&'b self)->&'b TarAddress {
        &self.client_addr
    }
}

impl TarSocketMonitor {
    pub fn new()->Self {
        TarSocketMonitor {
            fila:Arc::new(filament::Filament::new())
        }

    }

    pub fn bind_as_client(&self,addr:TarAddress,listener:Box<dyn TarListener + Send + Sync>) {
        let ll = Arc::new(listener);
        let fila_closure = self.fila.clone();
        
        self.fila.execute_closure(move || {
            let stream_result = 
                fila_closure.wait_future(async {
                    tokio::net::TcpStream::connect(addr.to_string()).await
                });

            match stream_result {
                Err(_) => {
                    return;},
                Ok(tcp_stream) => {
                    let addr = tcp_stream.peer_addr().unwrap();
                    let (rd,wr) = tcp_stream.into_split();
                    let connected_stream = Arc::new(Box::new(TarNetStream::new_with_address(
                                                                                                        rd,wr,fila_closure.clone(),
                                                                                                        String::from("client"),
                                                                                                        addr.ip().to_string(),
                                                                                                        addr.port() as u32)));
                    ll.clone().on_event(Event::Connect,None,connected_stream.clone());

                    loop {
                        let read_result = connected_stream.clone().read();
                        match read_result {
                            Some(data)=> {
                                ll.clone().on_event(Event::Message,Some(data),connected_stream.clone());
                            },
                            None => {
                                ll.clone().on_event(Event::Disconnect,None,connected_stream.clone());
                                return;
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn bind_as_server(&self,addr:TarAddress,listener:Box<dyn TarListener+Send+Sync>) {
        let ll = Arc::new(listener);
        let fila_closure = self.fila.clone();
        self.fila.clone().execute_closure(move || {
            let listen_result = fila_closure.wait_future(async {
                tokio::net::TcpListener::bind(addr.to_string()).await
            });

            match listen_result {
                Ok(listener) => {
                    loop {
                        let stream_result = fila_closure.wait_future( async {
                            listener.accept().await
                        });

                        match stream_result {
                            Ok((tcp_stream,addr)) => {
                                let (rd,wr) = tcp_stream.into_split();
                                let connected_stream = Arc::new(Box::new(TarNetStream::new_with_address(rd,
                                                                                                                                      wr,
                                                                                                                                      fila_closure.clone(),
                                                                                                                                      String::from("server"),
                                                                                                                                      addr.ip().to_string(),
                                                                                                                                      addr.port() as u32)));
                                ll.clone().on_event(Event::NewClient, None, connected_stream.clone());
                                    loop {
                                        let read_result = connected_stream.clone().read();
                                        match read_result {
                                            Some(data)=> {
                                                ll.clone().on_event(Event::Message,Some(data),connected_stream.clone());
                                            },
                                            None => {
                                                ll.clone().on_event(Event::Disconnect,None,connected_stream.clone());
                                                return;
                                            }
                                        }
                                    }
                            },
                            Err(_) =>{}
                        }
                    }
                    
                },
                Err(_) => {
                    //TODO
                }
            }
        });
    }
}