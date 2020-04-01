#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use {core::panic::PanicInfo};

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assetion... ");
    assert_eq!(1,1);
    println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("BlogOS started");
    println!("In QEMU Curses used ESC-1 and ESC-2 to switch between console and monitor");
    println!("Hello, from {}", "Brian");

    #[cfg(test)]
    test_main();

    loop {}
}
