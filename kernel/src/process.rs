// Process will contain a list of system calls that a process can run to communicate
// with the kernel.
use crate::{debug};

#[derive(Debug)]
pub struct Process {
    _id: u64,
    _active: bool,
}

pub fn syscall_interrupt(call_number: u64, return_address: u64, saved_rflags: u64, arg: &[u64; 6]) {
    debug!("syscall called {}", call_number);
}
