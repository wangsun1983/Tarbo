use crate::process::shmem;

pub fn do_test1() {
    let mut shmem = shmem::TarShareMemory::create(String::from("abc123"), 16);
    let v = [0x1,0x2,0x3];
    shmem.write(&v);

    let mut v2 = [0;3];
    shmem.read(&mut v2);
    for i in v2 {
        println!("v is {}", i);
    }
}