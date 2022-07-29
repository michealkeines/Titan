#![no_std] // dont link standar lib to the binary
#![no_main] // no main runtime

use core::panic::PanicInfo;
mod vga_buffer;


#[panic_handler] // as we dont have a standard lib, we have define how our code will panic 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//static HELLO:&[u8] = b"Hello World!";

#[no_mangle] // rust will automatically add unique name to function in the binary, we can stop that by explicitly mentioning that we dont have to do that
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers : {} {}", 42, 33.2).unwrap();

    //println!("Hello world {}, {}","!", 24);
    println!("$micheal@titan>> whomai");
    println!("micheal");
    loop {}
   // panic!("here in main");
   // loop {}
}

