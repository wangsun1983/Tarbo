//---- ByteRingArray ----
pub struct TarByteRingArray {
    next:usize,
    data:[u8;1024*32],
    size:usize,
    capacity:usize
}

impl TarByteRingArray {
    pub fn new()->TarByteRingArray {
        TarByteRingArray {
            next:0,
            data:[0;1024*32],
            size:0,
            capacity:1024*32
        }
    }

    pub fn push(&mut self,data:&[u8])->Result<u32,u32> {
        if data.len() > self.capacity - self.size {
            return Err(0);
        }

        if self.next + data.len() < self.capacity {
            self.data[self.next..self.next + data.len()].copy_from_slice(data);
        } else {
            self.data[self.next..self.capacity ].copy_from_slice(&data[0..(self.capacity-self.next)]);
            self.data[0..(data.len()-(self.capacity-self.next))].copy_from_slice(&data[self.capacity-self.next..(data.len()-(self.capacity-self.next))])
        }

        self.size += data.len();
        self.next = (self.next + data.len()) % self.capacity;
        Ok(0)
    }

    pub fn pop(&mut self,len:usize)->Result<Vec<u8>,u32> {
        if self.size < len {
            return Err(0);
        }

        let mut data:Vec<u8> = vec![0;len];
        let start = self.getStartIndex();
        //println!("start is {},len is {},self.data size is {},dat size is {}",
        //    start,len,self.data.len(),data.len());

        if start + len < self.capacity {
            data.copy_from_slice(&self.data[start..start + len]);            
        } else {
            data[0..].copy_from_slice(&self.data[start..(self.capacity - start)]);
            data[(self.capacity - start)..].copy_from_slice(&self.data[0..(self.size - (self.capacity - self.next))]);
        }

        self.size -= len;
        Ok(data)
    }

    pub fn getU32(&mut self)->Result<u32,u32> {
        if self.size < 4 {
            return Err(0);
        }

        let mut value:u32 = 0;
        let mut start = self.getStartIndex();
        for i in 0..4 {
            value |= (self.data[start] as u32) << i;
            start = (start + 1)%self.capacity;
        }
        self.size = self.size - 4;
        Ok(value)
    }

    pub fn getStoredDataSize(&self) ->usize{
        self.size
    }

    fn getStartIndex(&self)->usize {
        (self.next - self.size + self.capacity)%self.capacity
    }

    

}