#![no_std] // dont link standard lib to the binary
#![no_main] // no main runtime
#![feature(custom_test_frameworks)]
#![test_runner(titan_os::test_runner)] // this takes all the function that are marked as test and calles run in them
#![reexport_test_harness_main = "test_main"] // this is to tell the custom test framework to use this as it main entry function

use core::panic::PanicInfo;
use titan_os::println;

#[cfg(not(test))] // Panic handler for normal excutable
#[panic_handler] // as we dont have a standard lib, we have define how our code will panic 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)] // Panic hanlder for test executabless
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    titan_os::test_panic_handler(info)
}

#[no_mangle] // rust will automatically add unique name to function in the binary, we can stop that by explicitly mentioning that we dont have to do that
pub extern "C" fn _start() -> ! {
    println!("$micheal@titan>> whomai");
    println!("micheal");

    // This is the main method for custom test exe
    #[cfg(test)]
    test_main();

    loop {}
   // panic!("here in main");
   // loop {}
}


/*
old _start
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers : {} {}", 42, 33.2).unwrap();*/
