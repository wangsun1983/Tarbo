use tokio::net::TcpStream;

use crate::net::address;
use crate::coroutine::filament::Filament;

use super::socketoutputstream::TarSocketOutputStream;
use super::socketinputstream::TarSocketInputStream;
use super::address::TarAddress;

use std::sync::Arc;

pub struct TarSocketBuilder {
    m_fila:Arc<Filament>,
}

impl TarSocketBuilder {
    pub fn new()->Self {
        TarSocketBuilder {
            m_fila:Arc::new(Filament::new())
        }
    }

    pub fn create_socket(&self,addr:TarAddress)-> TarSocket {
        TarSocket {
            m_addr:addr,
            m_out:None,
            m_in:None,
            m_fila:self.m_fila.clone()
        }
    }

    pub fn get_fila(&self)->Arc<Filament> {
        self.m_fila.clone()
    }

    pub fn set_fila(&mut self,fila:Arc<Filament>) {
        self.m_fila = fila;
    }
}

pub struct TarSocket {
    m_addr:address::TarAddress,
    m_out:Option<Arc<TarSocketOutputStream>>,
    m_in:Option<Arc<TarSocketInputStream>>,
    m_fila:Arc<Filament>
}

impl TarSocket {
    pub fn connect(&mut self)->bool {
        let addr = self.m_addr.to_string();
        let stream_result = 
                self.m_fila.wait_future(async {
                    tokio::net::TcpStream::connect(addr.to_string()).await
                });

        match stream_result {
            Err(_) => {
                println!("connect fail", );
                return false;
            },
            Ok(tcp_stream) => {
                println!("connect success", );
                let (rd,wr) = tcp_stream.into_split();
                self.m_in = Some(Arc::new(TarSocketInputStream::new(rd,self.m_fila.clone())));
                self.m_out = Some(Arc::new(TarSocketOutputStream::new(wr,self.m_fila.clone())));
            }
        }
        true
    }

    pub async fn connect_async(&mut self)->bool {
        let addr = self.m_addr.to_string();
        let stream_result = tokio::net::TcpStream::connect(addr.to_string()).await;

        match stream_result {
            Err(_) => {
                println!("connect fail", );
                return false;
            },
            Ok(tcp_stream) => {
                println!("connect success", );
                let (rd,wr) = tcp_stream.into_split();
                self.m_in = Some(Arc::new(TarSocketInputStream::new(rd,self.m_fila.clone())));
                self.m_out = Some(Arc::new(TarSocketOutputStream::new(wr,self.m_fila.clone())));
            }
        }
        true
    }

    pub fn accept(&self)->Option<TarSocket> {
        self.m_fila.wait_future( async {
            self.accept()
        })
    }

    pub async fn accept_async(&self)->Option<TarSocket> {
        let bind_result = tokio::net::TcpListener::bind(self.m_addr.to_string()).await;
        match bind_result {
            Ok(tcp_listener)=> {
                let accept_result = tcp_listener.accept().await;
                match accept_result {
                    Ok((tcp_stream,addr)) => {
                        let (rd,wr) = tcp_stream.into_split();
                        let s = TarSocket {
                            m_addr:TarAddress::new(&addr.ip().to_string(),addr.port() as u32),
                            m_out:Some(Arc::new(TarSocketOutputStream::new(wr,self.m_fila.clone()))),
                            m_in:Some(Arc::new(TarSocketInputStream::new(rd,self.m_fila.clone()))),
                            m_fila:self.m_fila.clone()
                        };
                        return Some(s);
                    },
                    Err(_)=>{
                    }
                }
            },
            Err(err) =>{
            }
        }
        return None;
    }

    pub fn get_output_stream(&self)->Arc<TarSocketOutputStream> {
        self.m_out.clone().unwrap()
    }

    pub fn get_input_stream(&self)->Arc<TarSocketInputStream> {
        self.m_in.clone().unwrap()
    }

}