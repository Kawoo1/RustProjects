#![no_std]
#![no_main]

//Author: Kyle Shanahan
//This program is a simple example of the kernel. It prints "Hello, World!" to the VGA buffer
//in the 0xb8000 address. A kernal is a program that runs in the memory of the CPU.
use core::panic::PanicInfo;

// Entry point for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Main function for the kernel
    // Implement your kernel logic here

    // In this example, print "Hello, World!" to the VGA buffer
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in b"Hello, Kernal!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0F; // Text color attribute
        }
    }

    // Loop indefinitely
    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Implement your panic handling logic here
    loop {}
}

