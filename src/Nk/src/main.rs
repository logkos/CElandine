#![no_std]
#![no_main] 
#![allow(non_snake_case)]
pub mod api;
pub mod arch;
pub mod memory;
pub mod misc;
pub mod process;
pub mod bindings;

#[no_mangle]
pub extern "C" fn KernelInit() -> ! {
    loop {}
}

