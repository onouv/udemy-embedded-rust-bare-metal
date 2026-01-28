#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod mcu;
mod startup_stm32f303;
mod utils;

use board::{BLUE_LED, USER_BTN};
use core::panic::PanicInfo;

const APP_INIT_FAILED: &str = "application init failed.";

#[unsafe(no_mangle)]
fn main() {
    unsafe {
        init_application();
    }

    loop {}
}

unsafe fn init_application() {
    unsafe {
        BLUE_LED.init().expect(APP_INIT_FAILED); // panic if we cannot init ourselves
        BLUE_LED.on().expect(APP_INIT_FAILED);
    }
}

fn exti0_handler() {
    BLUE_LED.toggle();
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    // TODO: on panic, try to turn on a red LED somehow

    // TODO: on panic: log the panic message somewhere

    // 3. good bye
    loop {}
}
