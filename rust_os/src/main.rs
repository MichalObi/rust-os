#![feature(panic_implementation)]

/*
    Firstly we disabled Rust standard library - we are writing our own OS, so we can't use
    OS-based fn like handling thread or files system etc. .
*/

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points - no main fn runtime


use core::panic::PanicInfo; // providing info about panic - line of broken code and optional msg

// ! will will ensure that panic will never return anything
#[panic_handler] // implement fn that will be called on panic
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// never return
#[no_mangle] // this is important to find _start fn
pub extern "C" fn _start() -> ! { // _start is entry point search by linker (_start before main()) 
    loop {}
}
