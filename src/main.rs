#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use {blog_os::println, core::panic::PanicInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn print_startup_msg() {
    println!("BlogOS started");
    println!("In QEMU Curses used ESC-1 and ESC-2 to switch between console and monitor");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_startup_msg();
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
