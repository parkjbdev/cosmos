OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")

PAGE_SIZE = 64K;
PAGE_MASK = PAGE_SIZE - 1;

ENTRY(_start)
/* Qemu Device Tree Blob is 1MB */
kernel_start = 0x40100000;

PHDRS
{
  segment_ro PT_LOAD FLAGS(4); /* 4 == RO */
  segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
  segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
  . = kernel_start;
  . = ALIGN(PAGE_SIZE);
  __text_start = .;
  .text : ALIGN(8) {
    KEEP(*(.text._start))
    *(.text._start_cosmos)
    *(.text*)
    . = ALIGN(8);
  } :segment_rx
  . = ALIGN(PAGE_SIZE);
  __text_end = .;

  __rodata_start = .;
  .rodata : ALIGN(8) {
    *(.rodata)
    *(.rodata.*)
  } :segment_ro
  . = ALIGN(PAGE_SIZE);
  __rodata_end = .;

  /***********************************************************************************************
  * Data and BSS
  * **********************************************************************************************/

  __data_start = .;
  .data   : ALIGN(8) {
    *(.data)
    *(.data.*)
  } :segment_rw
  . = ALIGN(PAGE_SIZE);
  __data_end = .;

  __bss_start = .;
  .bss (NOLOAD)    : ALIGN(16) {
    *(.bss)
    *(.bss.*)
    . = ALIGN(16);
  } :segment_rw
  . = ALIGN(PAGE_SIZE);
  __bss_end = .;

  .got    : ALIGN(8) {
       /* Global offset table Todo */
    *(.got)
  } :segment_ro
  ASSERT(SIZEOF(.got) == 0, "Relocation Not Supported")
  . = ALIGN(PAGE_SIZE);

  /***********************************************************************************************
   * Boot Core Stack
   ***********************************************************************************************/
  __boot_core_stack_start = .;         /*   ^             */ 
                                       /*   | stack       */
  . += 16K;                            /*   | growth      */
                                       /*   | direction   */
  . = ALIGN(PAGE_SIZE);                /*   |             */
  __boot_core_stack_end = .;           /*   |             */
  kernel_end = .;
}
