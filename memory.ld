/* define entry point for execution */
ENTRY(reset_handler);

/* load linker default mappings for ISR vectors */
INCLUDE "device_STM32F303.x"

/* specify memory areas */
/* refer to RM0316, Sect. 3.2.2, Table 4*/
MEMORY
{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K  /* Load Memory */
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 12K   /* Volatile Memory */
    CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 8K /* CPU coupled RAM */
}

/* helper symbols */
_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

SECTIONS {
    /* executable code */
    .text : {
      . = ORIGIN(FLASH);
      LONG(_start_of_stack);
      KEEP(*(.vector_table));
      *(.text);
      *(.text*);
      . = ALIGN(4);
    } > FLASH

    /* constant data */
    .rodata : {
      . = ALIGN(4);
      *(.rodata);
      *(.rodata*);
      . = ALIGN(4);
    } > FLASH

    /* initialized data from FLASH to RAM */
    .data : {
        . = ALIGN(4);
        _start_lma_data = LOADADDR(.data); /* start of LMA for .data section */
        _start_vma_data = .; /* start of VMA for .data section */
        *(.data);
        *(.data*);
        . = ALIGN(4);
        _end_vma_data = .; /* end of VMA for .data section */

    } > RAM AT> FLASH

    /* uninitialized data, to be init'd at startup */
    .bss : {
      _start_vma_bss = .;
      . = ALIGN(4);
      *(.bss);
      *(.bss*);
      . = ALIGN(4);
      _end_vma_bss = .;
    } > RAM

    /* link-time check to fail, if heap and stack should not fit in RAM */
    .ram_size_check : {
        . = ALIGN(8); /* for ARM-Cortex M4 stack */
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}