#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod mcu;
mod startup_stm32f303;
mod utils;

use board::{BLUE_LED, USER_BTN};
use core::panic::PanicInfo;


#[unsafe(no_mangle)]
fn main() {
    unsafe {
        init_application();
    }

    loop {}
}

unsafe fn init_application() {
    unsafe {
        BLUE_LED.init();
        BLUE_LED.on();
        USER_BTN.init();
    }
}

fn exti0_handler() {
    BLUE_LED.toggle();
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
