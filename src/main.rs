#![no_std]
#![no_main]
#![allow(clippy::empty_loop, unused)]

mod board;
mod mcu;
mod startup_stm32f303;
mod utils;

use board::LD4;
use core::panic::PanicInfo;

const LED_INIT_FAILED: &str = "led init failed.";
const LED_ON_FAILED: &str = "turning led on failed";

#[unsafe(no_mangle)]
fn main() {
    unsafe {
        init_application();
    }

    loop {}
}

unsafe fn init_application() {
    unsafe {
        LD4.init().expect(LED_INIT_FAILED); // panic if we cannot init ourselves
        LD4.on().expect(LED_ON_FAILED);
    }
}

fn exti0_handler() {
    LD4.toggle();
}

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
