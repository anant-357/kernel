#![no_std]
#![no_main]

mod vga;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static START: &[u8] = b"The Beginning!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;

    for (i, &byte) in START.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = byte;
            *vga.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
