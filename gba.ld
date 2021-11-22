
ENTRY(__asm_entry)

MEMORY
{
  /* Read Only Memory: most things */
  rom (rx)   : ORIGIN = 0x08000000, LENGTH = 32M
  
  /* Internal Work Ram: globals and select high perfomance code */
  iwram (wx) : ORIGIN = 0x03000000, LENGTH = 32K

  /* External Work Ram: no default usage */
  ewram (wx) : ORIGIN = 0x02000000, LENGTH = 256K
}

SECTIONS
{
  /* The rom starts with all the code. */
  .text : {
    . = ALIGN(4);

    /* The assembly runtime is the *most* first of all our code. */
    KEEP(rt0.o(.text*));
    
    /* Next is all other code */
    *(.text .text.*);
    
    . = ALIGN(4);
  } >rom =0xAA

  /* Any large blobs of constant data go next in the rom. */
  .rodata : {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom =0xAA

  /* initialized global variables and functions that we copy to iwram */
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
  
  /* blank variables that we just zero in iwram */
  .bss : {
    . = ALIGN(4);
    __bss_start = ABSOLUTE(.);
    *(.bss .bss.*);
    . = ALIGN(4);
    __bss_end = ABSOLUTE(.);
  } >iwram AT>rom =0xAA

  /* ewram content */
  . = ALIGN(4);
  __ewram_position_in_rom = .;
  .ewram : {
    . = ALIGN(4);
    __ewram_start = ABSOLUTE(.);
    *(.ewram .ewram.*);
    . = ALIGN(4);
    __ewram_end = ABSOLUTE(.);
  } >ewram AT>rom =0xAA
}
