#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod framebuffer;
pub mod qemu;
pub mod serial;

use core::panic::PanicInfo;

use crate::qemu::{exit_qemu, QemuExitCode};

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        sprint!("{}...\t", core::any::type_name::<T>());
        self();
        sprintln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    sprintln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    sprintln!("[failed]\n");
    sprintln!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
