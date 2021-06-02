#![feature(abi_efiapi)]
#![no_std]
#![no_main]
use uefi::prelude::*;
// use uefi::CStr16;
// use utf16_literal::utf16;
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    let stdout = st.stdout();
    stdout.clear()
        .warning_as_error()
        .map_err(|_| core::fmt::Error).unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}
