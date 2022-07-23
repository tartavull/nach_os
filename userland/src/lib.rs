#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

pub mod shell;

use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_error("[failed]\n");
    write_error("Error \n");
    exit(1);
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    for test in tests {
    }
}

// system calls:
pub fn write_msg(msg: &str) {
    //this sommehow has to trigger an interrupt
    //so that the kernel will be called 
    //and print this message for us
    // See https://stackoverflow.com/questions/18717016/what-are-ring-0-and-ring-3-in-the-context-of-operating-systems
    // And https://github.com/vinaychandra/MoonDustKernel/blob/master/src/common/memory/paging.rs
    use core::arch::asm;
    unsafe {
        asm!(
            "mov rax, 1",
            "int 0x80",
        );
    }
    // This triggers the invalid opcode exception
}

pub fn write_error(msg: &str) {
}

pub fn exit(code: u8) {
}
