#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

use core::panic::PanicInfo;

mod board;
mod button;
mod led;
mod mcu;
mod startup_stm32f303;

use board::*;
use button::Button;
use led::Led;

// TODO: Make these thread-safe with Box<>, RC<> or the like
const BLUE_LED: &Led = &board::BLUE_LED;
const USER_BTN: &Button = &board::USER_BUTTON;

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
