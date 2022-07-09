#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod shell;


// system calls:
pub fn write_msg(msg: &str) {
    //this sommehow has to trigger an interrupt
    //so that the kernel will be called 
    //and print this message for us
    use core::arch::asm;
    unsafe {
        asm!("nop");
    }
    // This triggers the invalid opcode exception
}

pub fn write_error(msg: &str) {
}
