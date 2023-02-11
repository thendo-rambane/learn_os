#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
  write!(
    vga_buffer::WRITER.lock(),
    ", some numbers: {} {}",
    42,
    1.337
  )
  .unwrap();

  loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
  loop {}
}
