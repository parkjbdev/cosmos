OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")

__PAGE_SIZE = 64K;
PAGE_MASK = __PAGE_SIZE - 1;

ENTRY(_start)
kernel_start = 0x40200000;

PHDRS
{
  segment_ro PT_LOAD FLAGS(4); /* 4 == RO */
  segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
  segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
  . = kernel_start;

  .text : {
    __text_start = .;
    KEEP(*(.text._start))
    *(.text._start_cosmos)
    *(.text*)
    __text_end = .;
  } :segment_rx

  .rodata : ALIGN(8) {
    __rodata_start = .;
    PAGE_SIZE = .;
    QUAD(__PAGE_SIZE);
    *(.rodata)
    *(.rodata.*)
    __rodata_end = .;
  } :segment_ro

  . = ALIGN(__PAGE_SIZE);

  .data   : ALIGN(8) {
    *(.data)
    *(.data.*)
  } :segment_rw

  .bss (NOLOAD)    : ALIGN(16) {
    __bss_start = .;
    *(.bss)
    *(.bss.*)
    . = ALIGN(16);
    __bss_end_exclusive = .;
  } :segment_rw

  .got    : ALIGN(8) {
       /* Global offset table Todo */
    *(.got)
  } :segment_ro
  ASSERT(SIZEOF(.got) == 0, "Relocation Not Supported")

  . = ALIGN(4K); /* Align to page boundary */

  /***********************************************************************************************
   * Boot Core Stack
   ***********************************************************************************************/
  __boot_core_stack_start = .;         /*   ^             */ 
                                       /*   | stack       */
  . += 16K;                            /*   | growth      */
                                       /*   | direction   */
  __boot_core_stack_end_exclusive = .; /*   |             */
  kernel_end = .;
}
