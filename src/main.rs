#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod startup_stm32f303;
mod utils;

use board::BOARD;
use core::panic::PanicInfo;

use crate::board::Board;
const LED_INIT_FAILED: &str = "led init failed.";
const LED_ON_FAILED: &str = "turning led on failed";

#[unsafe(no_mangle)]
fn main() {
    let board = Board::new();
    let led4 = board.take_led(4).unwrap();
    led4.on();

    loop {}
}

fn exti0_handler() {}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // TODO: on panic, try to turn on a red LED somehow

    // TODO: on panic: log the panic message somewhere
    //let _msg = info.message().as_str().unwrap_or_else(|| {
    //    loop {}
    //});

    // 3. good bye
    loop {}
}
