#![no_std]
#![no_main]

use crate::qemu::{exit_qemu, QemuExitCode};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init_writer(boot_info);
    for i in 1..200 {
        println!("Hello");
    }
    loop {}
    exit_qemu(QemuExitCode::Success);
}

entry_point!(kernel_main);
