#![no_std]
#![no_main]

mod vga;
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!(" The Beginning {:#?}", boot_info);
    loop {}
}

entry_point!(kernel_main);
