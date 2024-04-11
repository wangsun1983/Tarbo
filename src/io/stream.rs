use std::io;

pub trait TarIoStream {
    fn read(&mut self,buff:&mut [u8])->io::Result<usize>;
    fn write(&mut self,buff:&[u8])->io::Result<usize>;
    fn seek_to(&mut self,index:u64) ->bool;
}