// Process will contain a list of system calls that a process can run to communicate
// with the kernel.
use crate::{debug, error, print};
use core::slice;
use core::str;

#[derive(Debug)]
pub struct Process {
    _id: u64,
    _active: bool,
}

pub fn syscall_interrupt(call_number: u64, arg: &[u64; 2]) {
    debug!("syscall\n    type: {}\n    args:{:?}",
           call_number, arg); 
    write(arg[0], arg[1]);
}

pub fn write(arg0: u64, arg1: u64) {
    let ptr = arg0 as *const u8 ;
    let len = arg1 as usize;

    let slice = unsafe {
        slice::from_raw_parts(ptr, len)
    };
    let s = str::from_utf8(slice);
    match s {
        Ok(v) => print!("{}",v),
        Err(e) => error!("{e:?}"),
    }
}
