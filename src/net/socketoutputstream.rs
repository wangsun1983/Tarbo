use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::coroutine::filament;

pub struct TarSocketOutputStream {
    m_stream:tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>,
    m_fila:Arc<filament::Filament>,
}

impl TarSocketOutputStream {
    pub fn new(writestream:tokio::net::tcp::OwnedWriteHalf,
            fila:Arc<filament::Filament>)->Self {
        TarSocketOutputStream {
            m_stream:tokio::sync::Mutex::new(writestream),
            m_fila:fila
        }
    }

    pub fn write(&self,buf:&[u8])->Option<usize> {
        self.m_fila.wait_future(async {
            self.write_async(buf).await
        })
    }

    pub async fn write_async(&self,buf:&[u8])->Option<usize> {
        let mut lock_obj = self.m_stream.lock().await;
        let result = lock_obj.write(buf).await;
        match result {
            Ok(size) => {
                return Some(size);
            },
            Err(_) => {
            }
        }
        return None;
    }

    pub async fn close_async(&self) {
        let mut lock = self.m_stream.lock().await;
        lock.shutdown().await;
    }

}