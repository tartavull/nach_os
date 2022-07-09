#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(kernel::test_runner)]

use core::panic::PanicInfo;
use kernel::serial_println;
use kernel::test_framework::test_panic_handler;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    serial_println!("test_println output");
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
