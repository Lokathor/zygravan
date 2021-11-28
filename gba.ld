
ENTRY(__asm_entry)

MEMORY
{
  rom (rx)   : ORIGIN = 0x08000000, LENGTH = 32M
  iwram (wx) : ORIGIN = 0x03000000, LENGTH = 32K
  ewram (wx) : ORIGIN = 0x02000000, LENGTH = 256K
}

SECTIONS
{
  .text.header : {
    . = ALIGN(4);
    KEEP(rt0.o(.text.header));
    . = ALIGN(4);
  } >rom =0xFF

  .text : {
    . = ALIGN(4);
    KEEP(rt0.o(.text*));
    *(.text .text.*);
    . = ALIGN(4);
  } >rom =0xAA

  .rodata : {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom =0xAA

  . = ALIGN(4);
  __data_position_in_rom = .;
  .data : {
    . = ALIGN(4);
    __data_start = ABSOLUTE(.);
    *(.data .data.*);
    *(.iwram .iwram.*);
    . = ALIGN(4);
    __data_end = ABSOLUTE(.);
  } >iwram AT>rom =0xAA
  
  .bss : {
    . = ALIGN(4);
    __bss_start = ABSOLUTE(.);
    *(.bss .bss.*);
    . = ALIGN(4);
    __bss_end = ABSOLUTE(.);
  } >iwram AT>rom =0xAA

  . = ALIGN(4);
  __ewram_position_in_rom = .;
  .ewram : {
    . = ALIGN(4);
    __ewram_start = ABSOLUTE(.);
    *(.ewram .ewram.*);
    . = ALIGN(4);
    __ewram_end = ABSOLUTE(.);
  } >ewram AT>rom =0xAA

  /DISCARD/ : {
    *(.ARM.exidx.*)
  }
}
