
use core::ptr;

//
// Promise unused exception handlers to rustc,
// ISR vector table can be filled
//
unsafe extern "C" {
    //fn reset_handler();
    //fn nmi_handler();
    //fn hardfault_handler();
    //fn mem_manage_handler();
    //fn busfault_handler();
    //fn usagefault_handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC1_2_Handler();
    fn ADC3_Handler();
    fn ADC4_Handler();
    fn CAN_RX1_Handler();
    fn CAN_SCE_Handler();
    fn COMP123_Handler();
    fn COMP456_Handler();
    fn COMP7_Handler();
    fn DMA1_CH1_Handler();
    fn DMA1_CH2_Handler();
    fn DMA1_CH3_Handler();
    fn DMA1_CH4_Handler();
    fn DMA1_CH5_Handler();
    fn DMA1_CH6_Handler();
    fn DMA1_CH7_Handler();
    fn DMA2_CH1_Handler();
    fn DMA2_CH2_Handler();
    fn DMA2_CH3_Handler();
    fn DMA2_CH4_Handler();
    fn DMA2_CH5_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_TSC_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FLASH_Handler();
    fn FMC_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_EXTI23_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_EXTI24_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn PVD_Handler();
    fn RCC_Handler();
    fn RTCAlarm_Handler();
    fn RTC_WKUP_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn SPI4_Handler();
    fn TAMP_STAMP_Handler();
    fn TIM1_BRK_TIM15_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_TIM17_Handler();
    fn TIM1_UP_TIM16_Handler();
    fn TIM20_BRK_Handler();
    fn TIM20_CC_Handler();
    fn TIM20_TRG_COM_Handler();
    fn TIM20_UP_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM6_DACUNDER_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_Handler();
    fn TIM8_UP_Handler();
    fn UART4_EXTI34_Handler();
    fn UART5_EXTI35_Handler();
    fn USART1_EXTI25_Handler();
    fn USART2_EXTI26_Handler();
    fn USART3_EXTI28_Handler();
    fn USB_HP_CAN_TX_Handler();
    fn USB_HP_Handler();
    fn USB_LP_CAN_RX0_Handler();
    fn USB_LP_Handler();
    fn USB_WKUP_EXTI_Handler();
    fn USB_WKUP_Handler();
    fn WWDG_Handler();
}

// promise linker-generated memory indices to rustc 
// note mut types only to enable comparing with other mut
unsafe extern "C" {
    static _start_lma_data: u32;
    static mut _start_vma_data: u32;
    static mut _end_vma_data: u32;
    static mut _start_vma_bss: u32;
    static mut _end_vma_bss: u32;
}

//
// ISR vector table for STM32F303
//
#[used]
#[unsafe(link_section = ".vector_table")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 100] = [
    Some(reset_handler),
    Some(nmi_handler),
    Some(hardfault_handler),
    Some(mem_manage_handler),
    Some(busfault_handler),
    Some(usagefault_handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_TSC_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_CH1_Handler),
    Some(DMA1_CH2_Handler),
    Some(DMA1_CH3_Handler),
    Some(DMA1_CH4_Handler),
    Some(DMA1_CH5_Handler),
    Some(DMA1_CH6_Handler),
    Some(DMA1_CH7_Handler),
    Some(ADC1_2_Handler),
    Some(USB_HP_CAN_TX_Handler),
    Some(USB_LP_CAN_RX0_Handler),
    Some(CAN_RX1_Handler),
    Some(CAN_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM15_Handler),
    Some(TIM1_UP_TIM16_Handler),
    Some(TIM1_TRG_COM_TIM17_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_EXTI23_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_EXTI24_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_EXTI25_Handler),
    Some(USART2_EXTI26_Handler),
    Some(USART3_EXTI28_Handler),
    Some(EXTI15_10_Handler),
    Some(RTCAlarm_Handler),
    Some(USB_WKUP_Handler),
    Some(TIM8_BRK_Handler),
    Some(TIM8_UP_Handler),
    Some(TIM8_TRG_COM_Handler),
    Some(TIM8_CC_Handler),
    Some(ADC3_Handler),
    Some(FMC_Handler),
    None,
    None,
    Some(SPI3_Handler),
    Some(UART4_EXTI34_Handler),
    Some(UART5_EXTI35_Handler),
    Some(TIM6_DACUNDER_Handler),
    Some(TIM7_Handler),
    Some(DMA2_CH1_Handler),
    Some(DMA2_CH2_Handler),
    Some(DMA2_CH3_Handler),
    Some(DMA2_CH4_Handler),
    Some(DMA2_CH5_Handler),
    Some(ADC4_Handler),
    None,
    None,
    Some(COMP123_Handler),
    Some(COMP456_Handler),
    Some(COMP7_Handler),
    None,
    None,
    None,
    None,
    None,
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(USB_HP_Handler),
    Some(USB_LP_Handler),
    Some(USB_WKUP_EXTI_Handler),
    Some(TIM20_BRK_Handler),
    Some(TIM20_UP_Handler),
    Some(TIM20_TRG_COM_Handler),
    Some(TIM20_CC_Handler),
    None,
    None,
    None,
    Some(SPI4_Handler),
];

//
// System exception handlers
//

// Note: this function is the ENTRY point announced to the linker (memory.ld).
#[unsafe(no_mangle)]
unsafe extern "C" fn reset_handler() {
    init_vma_data_section_from_lma();
    init_vma_bss_section();

    crate::main();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn nmi_handler() {
    loop {}
}


#[unsafe(no_mangle)]
unsafe extern "C" fn hardfault_handler() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn usagefault_handler() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn busfault_handler() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mem_manage_handler() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn default_handler() {
    loop {}
}

//
// helper functions
//

fn init_vma_data_section_from_lma() {
    unsafe {
        let mut lma_src= ptr::addr_of!(_start_lma_data);
        let mut vma_dest: *mut u32 = ptr::addr_of_mut!(_start_vma_data);

        // must be mut only to allow comparison with mut
        const VMA_END: *mut u32 = ptr::addr_of_mut!(_end_vma_data);
        while vma_dest < VMA_END {
            *vma_dest = *lma_src;
            vma_dest = vma_dest.add(1);
            lma_src = lma_src.add(1);
        }
    }
}

fn init_vma_bss_section() {
    unsafe {
        let mut vma_dest: *mut u32 = ptr::addr_of_mut!(_start_vma_bss);

        // must be mut only to allow comparison with mut
        const VMA_END: *mut u32 = ptr::addr_of_mut!(_end_vma_bss);
        while vma_dest < VMA_END {
            *vma_dest = 0;
            vma_dest = vma_dest.add(1);
        }
    }
}