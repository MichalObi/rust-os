#![feature(panic_implementation)]

/*
    Firstly we disabled Rust standard library - we are writing our own OS, so we can't use
    OS-based fn like handling thread or files system etc. .
*/

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points - no main fn runtime

mod vga_buffer; // load vga module from vga_buffer.rs file

extern crate bootloader_precompiled; // for kernel load

extern crate volatile; // prevent too agresive compiler optimization when writing to VGA buffer

use core::panic::PanicInfo; // providing info about panic - line of broken code and optional msg

// ! will will ensure that panic will never return anything
#[panic_handler] // implement fn that will be called on panic
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static (sort of global) byte string
static WELCOME: &[u8] = b"Welcome in Rust OS!";

// this is important to find _start fn
#[no_mangle]

// _start is entry point search by linker (_start before main())
// ! mean never return

pub extern "C" fn _start() -> ! {
    // address of vga memory location
    let video_memory_location = 0xb8000;

    // cyan color
    let letter_color = 0xb;

    // cast to "raw pointer" -> not so safe, but fast
    let vga_buffer = video_memory_location as *mut u8;

    for (i, &byte) in WELCOME.iter().enumerate() {
        // no guarantee that vga_buffer as raw pointer will be valid - but in this point we sure it does
        unsafe {
            // write letter of WELCOME word - 2 bytes for every cell
            *vga_buffer.offset(i as isize * 2) = byte;

            // give letter color
            *vga_buffer.offset(i as isize * 2 + 1) = letter_color;
        }
    }

    // let test_text = "Test text !";
    // vga_buffer::print_test_text(test_text);
    
    vga_buffer::print_with_macro_test();

    loop {}
}
