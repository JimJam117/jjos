#![no_std]
#![no_main]

use core::panic::PanicInfo;

// panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"JimJam OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // let colorsArr = [0xe, 0xc, 0xf, 0xa];
    let mut col = 0xa;
    for (i, &byte) in HELLO.iter().enumerate() {
        
        if (&[byte] == b" ") {
            col = 0xc;
        }
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = col;
        }
    }

    loop {}
}


