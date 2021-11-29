.global __asm_entry
.global RUST_IRQ_HANDLER

.arm

.section .text.header
__asm_entry:
  b asm_init
  
  @ gbafix fills the following blank space in with proper header data when we
  @ decide to make the full rom. Even though we are building a standard rom, we
  @ zero the area all the way out to the end of the multiboot header so that
  @ mGBA doesn't potentially get confused.
  
  .space 0xE0
.previous

asm_init:
  @ Pin the DMA3 base address for use in the next 2 operations.
  ldr  r4, =0x040000D4

copy_data:
  ldr r0, =__data_start
  ldr r1, =__data_end
  ldr r2, =__data_position_in_rom
  sub r3, r1, r0            @ r3 = __data_end - __data_start
  lsrs r3, r3, #2           @ r3 = r3 / 4
  beq 1f                    @ if r3 == 0, branch 1f
  str  r2, [r4]             @ set source address
  str  r0, [r4, #4]         @ set destination
  strh r3, [r4, #8]         @ set the count
  mov  r5, #0x8400          @ 32-bit transfers, DMA enabled
  strh r5, [r4, #10]        @ set the config bits
  1:

zero_bss:
  ldr r0, =__bss_start
  ldr r1, =__bss_end
  mov r4, #0
  sub r3, r1, r0
  lsrs r3, r3, #2
  1:
  beq 2f
  str r4, [r0]
  add r0, r0, #4
  subs r3, r3, #1
  b 1b
  2:

set_the_rt_handler:
  ldr r0, =rt_irq_handler
  ldr r1, =0x03FFFFFC
  str r0, [r1]

call_to_rust_main:
  ldr lr, =1f
  ldr r0, =main
  bx r0

  @ `main` should not return, but if it does we'll catch it.
  1: b 1b               @ end of program.

.section .iwram
.align 4
rt_irq_handler:
  mov   r0, #0x04000000 @ we could probably remove this and save 1 whole cycle.

  add   r2, r0, #0x200
  ldr   r0, [r2]
  and   r0, r0, r0, lsr #16
  strh  r0, [r2, #2]
  
  mov   r3, #0x04000000
  ldrh  r1, [r3, #-8]
  orr   r1, r1, r0
  strh  r1, [r3, #-8]
  
  ldr   r1, =RUST_IRQ_HANDLER
  ldr   r1, [r1]
  cmp   r1, #0
  beq   .L_end_of_rt_irq

  add   r2, r2, #8
  mov   r12, #0
  swp   r12, r12, [r2]
  mrs   r3, SPSR
  push  {r3}
  mov   r3, #0b11111
  msr   CPSR_cf, r3
  mov   r3, sp
  bic   sp, sp, #7
  push  {r2, r3, r12, lr}
  adr   lr, .L_after_the_call
  bx    r1
  .L_after_the_call:
  pop   {r2, r3, r12, lr}
  mov   sp, r3
  mov   r3, #0b10010010
  msr   CPSR_cf, r3
  pop   {r3}
  msr   SPSR, r3
  swp   r12, r12, [r2]
  
  .L_end_of_rt_irq:
  bx    lr
.previous

.section .bss
.align 4
RUST_IRQ_HANDLER:
  .zero 4
.previous

@ this label makes the disassembly look much better.
end_of_rt0_file:
