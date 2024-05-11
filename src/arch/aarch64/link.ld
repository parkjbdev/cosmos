/* Parts of this linker script are directly taken from Andre Richters Project:
 * https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/16_virtual_mem_part4_higher_half_kernel/src/bsp/raspberrypi/link.ld 
*/

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")
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
    *(.text)
    *(.text.*)
    __text_end = .;
  } :segment_rx

  .rodata : ALIGN(8) {
    __rodata_start = .;
    *(.rodata)
    *(.rodata.*)
    __rodata_end = .;
  } :segment_ro

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
  /* ASSERT(SIZEOF(.got) == 0, "Relocation Not Supported"); */

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
