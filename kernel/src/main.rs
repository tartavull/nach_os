#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nach_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate userland;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::println!("nachos");
    kernel::init();

    kernel::scheduler::create_process(&userland::shell::_start);

    kernel::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
