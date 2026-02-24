OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")

__PAGE_SIZE_ = 64K;
__PAGE_MASK_ = __PAGE_SIZE_ - 1;

ENTRY(_start)

PHDRS
{
  segment_ro PT_LOAD FLAGS(4); /* 4 == RO */
  segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
  segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
  . = 0x40000000;
  __kernel_start_ = .;

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

  .rela.dyn : ALIGN(8) {
    __rela_start_ = .;
    *(.rela .rela*)
    __rela_end_ = .;
  } :segment_ro

  .got : ALIGN(8) {
    *(.got)
    *(.got.plt)
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

  /DISCARD/ : {
    *(.dynamic)
    *(.dynsym)
    *(.dynstr)
    *(.hash)
    *(.gnu.hash)
    *(.plt)
    *(.plt.got)
    *(.interp)
    *(.note.GNU-stack)
    *(.note.gnu.property)
    *(.comment)
  }
}
