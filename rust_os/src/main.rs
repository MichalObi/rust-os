#![feature(panic_implementation)]

/*
    Firstly we disabled Rust standard library - we are writing our own OS, so we can't use
    OS-based fn like handling thread or files system etc. .
*/

#![no_std]

use core::panic::PanicInfo; // providing info about panic - line of broken code and optional msg

// turn off name mangling (aka. name decoration) - generate unique functions names by compiler
#[no_mangle]

// ! will will ensure that panic will never return anything
#[panic_handler] // implement fn that will be called on panic
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
