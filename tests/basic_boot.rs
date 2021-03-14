#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(nach_os::test_runner)]

use core::panic::PanicInfo;
use nach_os::serial_println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nach_os::test_panic_handler(info)
}


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    serial_println!("test_println output");
}
