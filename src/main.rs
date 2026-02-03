#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod mcu;
mod startup_stm32f303;
mod utils;

use board::BOARD;
use core::panic::PanicInfo;
use mcu::{GPIO, GPIOId, OutputType};
const LED_INIT_FAILED: &str = "led init failed.";
const LED_ON_FAILED: &str = "turning led on failed";

#[unsafe(no_mangle)]
fn main() {
    #[allow(static_mut_refs)] // ...since we are implementing a singleton pattern in take_port()
    unsafe {
        let pa0 = BOARD.take_port(GPIOId::A, 0).unwrap().to_input();
        pa0.init().expect("GPIOA init failed for pin 0");

        let pe8 = BOARD.take_port(GPIOId::E, 8).unwrap().to_output();
        pe8.init(OutputType::PushPull).expect("GPIOE init failed for pin 8");
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
