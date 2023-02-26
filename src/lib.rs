#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;
pub mod qemu;
pub mod test;

use qemu::{QemuExitCode, exit_qemu};
use core::panic::PanicInfo;

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[ Failed ]\n");
  serial_println!("Error: {}", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
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
  test_panic_handler(info);
  #[allow(unreachable_code)]
  loop {}
}
