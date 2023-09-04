#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    println!("PANIC!: {}", _info);
    loop {}
}

static greet: &[u8] = b"JimJam OS ";
static versionText: &[u8] = b" VERSION ";
static version: &[u8] = env!("CARGO_PKG_VERSION").as_bytes(); 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::print_something();
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    vga_buffer::WRITER.lock().write_str("JIMJAM OS").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    vga_buffer::WRITER.lock().write_str("Hello!").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "VGA buffer test").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    print!("PRINT TEST");
    print!(" - PRINT TEST 2");
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    println!("Print LN test: {}","success!");
    println!("Print LN test 2: {}","success!");

    let vga_buffer = 0xb8000 as *mut u8;

    let mut col = 0xa;
    let mut actualI = 0;

    for (i, &byte) in greet.iter().enumerate() {
        actualI += 1;
        unsafe {
            *vga_buffer.offset(actualI as isize * 2) = byte;
            *vga_buffer.offset(actualI as isize * 2 + 1) = col;
        }
    }
    
    // yellow line
    col = 0xe;
    actualI += 1;
    unsafe {
        *vga_buffer.offset(actualI as isize * 2) = b'-';
        *vga_buffer.offset(actualI as isize * 2 + 1) = col;
    }
    actualI += 1;
    unsafe {
        *vga_buffer.offset(actualI as isize * 2) = b'-';
        *vga_buffer.offset(actualI as isize * 2 + 1) = col;
    }
    actualI += 1;
    unsafe {
        *vga_buffer.offset(actualI as isize * 2) = b'-';
        *vga_buffer.offset(actualI as isize * 2 + 1) = col;
    }

    // set to red for version text
    col = 0xc;
    
    for (i, &byte) in versionText.iter().enumerate() {
        actualI += 1;
        unsafe {
            *vga_buffer.offset(actualI as isize * 2) = byte;
            *vga_buffer.offset(actualI as isize * 2 + 1) = col;
        }
    }

    // set to blue for version number
    col = 0x9;

    for (i, &byte) in version.iter().enumerate() {
        actualI += 1;
        unsafe {
            *vga_buffer.offset(actualI as isize * 2) = byte;
            *vga_buffer.offset(actualI as isize * 2 + 1) = col;
        }
    }

    panic!("test complete");

    loop {}
}

