#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"JimJam OS - - - VERSION 0.0.01";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    let vga_buffer = 0xb8000 as *mut u8;

    let mut col = 0xa;

    for (i, &byte) in HELLO.iter().enumerate() {
        
        if &[byte] == b"V" {
            col = 0xc;
        }
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = col;
        }
    }

    loop {}
}

