OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")

__PAGE_SIZE_ = 64K;
__PAGE_MASK_ = __PAGE_SIZE_ - 1;

ENTRY(_start)
/* Qemu Device Tree Blob is 1MB */
__kernel_start_ = 0x40000000;

PHDRS
{
  segment_ro PT_LOAD FLAGS(4); /* 4 == RO */
  segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
  segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
  . = __kernel_start_;
  __device_tree_start_ = .;
  . += 1M;
  __device_tree_end_ = .;

  __text_start_ = .;
  .text : ALIGN(65536) {
    KEEP(*(.text._start))
    *(.text._start_cosmos)
    *(.text*)
    . = ALIGN(65536);
  } :segment_rx
  . = ALIGN(__PAGE_SIZE_);
  __text_end_ = .;

  __rodata_start_ = .;
  .rodata : ALIGN(65536) {
    *(.rodata)
    *(.rodata.*)
  } :segment_ro
  . = ALIGN(__PAGE_SIZE_);
  __rodata_end_ = .;

  /***********************************************************************************************
  * Data and BSS
  * **********************************************************************************************/

  __data_start_ = .;
  .data   : ALIGN(65536) {
    *(.data)
    *(.data.*)
  } :segment_rw
  . = ALIGN(__PAGE_SIZE_);
  __data_end_ = .;

  __bss_start_ = .;
  .bss (NOLOAD)    : ALIGN(16) {
    *(.bss)
    *(.bss.*)
    . = ALIGN(16);
  } :segment_rw
  . = ALIGN(__PAGE_SIZE_);
  __bss_end_ = .;

  .got    : ALIGN(65536) {
       /* Global offset table Todo */
    *(.got)
  } :segment_ro
  ASSERT(SIZEOF(.got) == 0, "Relocation Not Supported")
  . = ALIGN(__PAGE_SIZE_);

  /***********************************************************************************************
   * Boot Core Stack
   ***********************************************************************************************/
  __boot_core_stack_start_ = .;         /*   ^             */
                                        /*   | stack       */
  . += 1M;                              /*   | growth      */
                                        /*   | direction   */
  . = ALIGN(__PAGE_SIZE_);              /*   |             */
  __boot_core_stack_end_ = .;           /*   |             */

  __mmio_remap_start_ = .;
  . += 1024M; /* 0x00000000 to 0x40000000 */
  __mmio_remap_end_ = .;

  . = ALIGN(__PAGE_SIZE_);
  . += 2048M;
  . = ALIGN(__PAGE_SIZE_);

  __kernel_end_ = .;

  /* 디버그 섹션들을 유지 */
    /* .debug_info 0 : { *(.debug_info) } */
    /* .debug_abbrev 0 : { *(.debug_abbrev) } */
    /* .debug_line 0 : { *(.debug_line) } */
    /* .debug_str 0 : { *(.debug_str) } */

    /* 또는 /DISCARD/에서 디버그 섹션을 제외 */
    /* /DISCARD/ : { */
    /*     *(.note.GNU-stack) */
    /*     *(.note.gnu.property) */
        /* 디버그 섹션은 제외 */
        /* *(.debug*) */
    /* } */
}
