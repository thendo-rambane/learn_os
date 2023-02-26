#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod test;
mod vga_buffer;
mod qemu;

use qemu::{QemuExitCode, exit_qemu};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  #[cfg(test)]
  test_main();

  loop {}
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(0, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[ Failed ]\n");
  serial_println!("Error: {}", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

