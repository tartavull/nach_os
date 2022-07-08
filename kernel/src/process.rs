// Process will contain a list of system calls that a process can run to communicate
// with the kernel.

#[derive(Debug)]
pub struct Process {
    id: u64,
    active: bool,
}
