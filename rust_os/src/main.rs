#![cfg_attr(test, allow(unused_imports))] // disabled lints about import
#![feature(panic_implementation)]

/*
    Firstly we disabled Rust standard library - we are writing our own OS, so we can't use
    OS-based fn like handling thread or files system etc. .
*/

/* if no test (cargo test) */

#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points - no main fn runtime

#[macro_use] // important annotation ! - import macros defined in vga_buffer with vga_buffer
mod vga_buffer; // load vga module from vga_buffer.rs file

extern crate bootloader_precompiled; // for kernel load

extern crate volatile; // prevent too agresive compiler optimization when writing to VGA buffer

extern crate spin; // introduce spinlock - can provide very simple lock (like Mutex in std)

extern crate core;

use core::panic::PanicInfo; // providing info about panic - line of broken code and optional msg

#[macro_use]
extern crate lazy_static; // lazly initialized static when acessed first time (instead of at compile time)

// ! will will ensure that panic will never return anything
#[panic_handler] // implement fn that will be called on panic
#[no_mangle]
#[cfg(not(test))] // compile only if test flag is not set (not compile when "cargo test" occured)
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// static (sort of global) byte string
static WELCOME: &[u8] = b"Welcome in Rust OS!";

// _start is entry point search by linker (_start before main())
// ! mean never return

#[cfg(not(test))] // compile only if test flag is not set (not compile when "cargo test" occured)
#[no_mangle] // this is important to find _start fn
pub extern "C" fn _start() -> ! {
    // address of vga memory location
    // let video_memory_location = 0xb8000;
    //
    // // cyan color
    // let letter_color = 0xb;
    //
    // // cast to "raw pointer" -> not so safe, but fast
    // let vga_buffer = video_memory_location as *mut u8;
    //
    // for (i, &byte) in WELCOME.iter().enumerate() {
    //     // no guarantee that vga_buffer as raw pointer will be valid - but in this point we sure it does
    //     unsafe {
    //         // write letter of WELCOME word - 2 bytes for every cell
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //
    //         // give letter color
    //         *vga_buffer.offset(i as isize * 2 + 1) = letter_color;
    //     }
    // }

    let test_text = "Test text ! \n";

    // vga_buffer::print_test_text(test_text);
    // vga_buffer::print_with_macro_test();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str(test_text);
    // write!(vga_buffer::WRITER.lock(), "Test numbers printing: {} {}", 21, 1.211).unwrap();

    println!("Hello World and ... {}, {}, test: {}", test_text, 2, "ababa");

    panic!("Some test panic msg");

    loop {}
}
