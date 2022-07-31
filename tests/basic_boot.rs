#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(titan_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use titan_os::println;

#[no_mangle]
pub extern  "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("this is a test print from basic boot.rs");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    titan_os::test_panic_handler(info)
}

