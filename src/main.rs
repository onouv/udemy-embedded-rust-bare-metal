#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod mcu;
mod startup_stm32f303;
mod utils;

use board::BOARD;
use core::panic::PanicInfo;
use mcu::gpio::{GPIO, GPIOId};
const LED_INIT_FAILED: &str = "led init failed.";
const LED_ON_FAILED: &str = "turning led on failed";

#[unsafe(no_mangle)]
fn main() {
    #[allow(static_mut_refs)] // ...since we are implementing a singleton pattern in take_port()
    let gpioe = unsafe { BOARD.take_port(GPIOId::A, 8).unwrap().to_input().unwrap() };
    unsafe {
        gpioe.init().expect("gpioe init failed");
    }
    
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
