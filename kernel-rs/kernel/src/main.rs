#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod framebuffer;
mod qemu;
mod serial;
use crate::qemu::{exit_qemu, QemuExitCode};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init_writer(boot_info);
    println!("Hello");
    #[cfg(test)]
    test_main();
    loop {}
    exit_qemu(QemuExitCode::Success);
}

entry_point!(kernel_main);

#[test]
fn trivial_assertion() {
    print!("trivial_assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
