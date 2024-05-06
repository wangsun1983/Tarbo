use std::sync::Arc;
use tokio::io::AsyncReadExt;

use crate::coroutine::filament;

pub struct TarSocketInputStream {
    m_stream:tokio::sync::Mutex<tokio::net::tcp::OwnedReadHalf>,
    m_fila:Arc<filament::Filament>,
}

impl TarSocketInputStream {
    pub fn new(readstream:tokio::net::tcp::OwnedReadHalf,
            fila:Arc<filament::Filament>)->Self {
        TarSocketInputStream {
            m_stream:tokio::sync::Mutex::new(readstream),
            m_fila:fila
        }
    }

    pub fn read_sync(&self)->Option<Vec<u8>> {
        self.m_fila.clone().wait_future( {
            self.read_async()
        })
    }

    pub async fn read_async(&self)->Option<Vec<u8>> {
        let mut buf:[u8;1024*4] = [0;1024*4];
        let mut lock_obj = self.m_stream.lock().await;
        let result = lock_obj.read(&mut buf).await;
        match result {
            Ok(size) => {
                return Some(Vec::from(&buf[0..size]));
            },
            Err(_) => {
            }
        }
        return None;
    }
}