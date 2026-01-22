#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

use core::panic::PanicInfo;

mod startup_stm32f303;

// initialized data -> .data
static mut SCORES: [i32; 5] = [1, 2, 3, 4, 5];

// constant data -> .rodata
const _NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];

// uninitialized array -> .bss
static mut _BUFFER: [u8; 1024] = [0; 1024];


#[unsafe(no_mangle)]
fn main() {

    let mut total_score = 0;

    unsafe {
        for score in SCORES {
            total_score += score;
        }
    }

    unsafe {
        _BUFFER[0] = 100;
    }

    loop {

    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}