use std::{num::NonZeroUsize, os::fd::RawFd};

use libc::{ftruncate, memcpy, write};
use nix::{fcntl::OFlag, sys::{mman::{mmap, munmap, shm_open, shm_unlink, MapFlags, ProtFlags}, stat::fstat}};
use nix::sys::stat::Mode;

pub struct TarShareMemory{
    m_fd:RawFd,
    m_size:usize,
    m_ptr:*mut libc::c_void
}

impl TarShareMemory {
    pub fn create(name:String,size:usize)->Self {
        TarShareMemory::default_open(name,size,true)
    }

    pub fn open(name:String)->Self {
        TarShareMemory::default_open(name,0,false)
    }

    pub fn write(&mut self,data:&[u8]) {
        unsafe {
            libc::memcpy(self.m_ptr, data.as_ptr() as *const libc::c_void, data.len());
        }
    }

    pub fn read(&mut self,data:&mut [u8]) {
        unsafe {
            libc::memcpy(data.as_ptr() as *mut libc::c_void,self.m_ptr,  data.len());
        }
    }

    fn default_open(name:String,size:usize,new_fd:bool)->Self {
        let mut flag = OFlag::O_RDWR;
        let mut shm_size = size;

        if new_fd {
            flag |= OFlag::O_CREAT|OFlag::O_EXCL;
            //if file exit,shm_unlink it
            shm_unlink(name.as_str());
        }

        let shem_result: Result<i32, nix::errno::Errno> = shm_open(name.as_str(),
                                            flag,
                                            Mode::S_IRUSR|Mode::S_IWUSR);
        match shem_result {
            Ok(fd)=> {
                if new_fd {
                    unsafe {
                        ftruncate(fd, size as i64);
                    }
                }
                unsafe {
                    if !new_fd {
                        shm_size = match fstat(fd) {
                            Ok(v)=> v.st_size as usize,
                            Err(_)=> panic!("fail to get shm file size")
                        }
                    }

                    let nz_map_size = NonZeroUsize::new(shm_size).unwrap();
                    let ptr: *mut libc::c_void = mmap(None,nz_map_size,ProtFlags::PROT_READ|ProtFlags::PROT_WRITE,MapFlags::MAP_SHARED,fd,0).unwrap();
                    TarShareMemory {
                        m_fd:fd,
                        m_size:shm_size,
                        m_ptr:ptr
                    }
                }
            },
            Err(reason)=> {
                panic!("ShareMemory open fail,reasion is {}",reason);

            }
        }
    }

}