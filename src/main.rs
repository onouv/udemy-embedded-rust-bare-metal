#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

use core::panic::PanicInfo;

mod startup_stm32f303;
mod led;
mod button;
mod board;
mod mcu;

use board::*;
use button::Button;

use crate::{button::ButtonStatus, led::Led};

const BLUE_LED: &Led = &board::BLUE_LED;

#[unsafe(no_mangle)]
fn main() {
    unsafe {
        init_application();
    }

    loop {}
}

unsafe fn init_application() {
    unsafe {
        led::init(&board::BLUE_LED);
        led::on(BLUE_LED);
        button::init(Button::User);
    }
}

fn exti0_handler() {
    led::toggle(BLUE_LED);
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}