#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::mem::size_of_val;

use zygravan::gba::*;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
  BACKDROP.write(Color::from_rgb(31, 0, 0));
  loop {}
}

/*
extern "C" fn irq_handler(_bits: IrqBits) {
  //
}
*/

#[no_mangle]
extern "C" fn main() -> ! {
  //
  decompress_cp437_data_to(BgCharblock::new(0).tiles4());

  //
  let pink = Color::from_rgb(28, 15, 15);
  for x in 0..16 {
    BG_PALETTE.index(0 + 16 * x).write(pink);
  }

  //
  BG_PALETTE.index(1 + 16 * 0).write(Color::BLACK);
  BG_PALETTE.index(1 + 16 * 1).write(Color::RED);
  BG_PALETTE.index(1 + 16 * 2).write(Color::GREEN);
  BG_PALETTE.index(1 + 16 * 3).write(Color::YELLOW);
  BG_PALETTE.index(1 + 16 * 4).write(Color::BLUE);
  BG_PALETTE.index(1 + 16 * 5).write(Color::MAGENTA);
  BG_PALETTE.index(1 + 16 * 6).write(Color::CYAN);
  BG_PALETTE.index(1 + 16 * 7).write(Color::WHITE);
  BG_PALETTE.index(1 + 16 * 8).write(Color::DIM_BLACK);
  BG_PALETTE.index(1 + 16 * 9).write(Color::DIM_RED);
  BG_PALETTE.index(1 + 16 * 10).write(Color::DIM_GREEN);
  BG_PALETTE.index(1 + 16 * 11).write(Color::DIM_YELLOW);
  BG_PALETTE.index(1 + 16 * 12).write(Color::DIM_BLUE);
  BG_PALETTE.index(1 + 16 * 13).write(Color::DIM_MAGENTA);
  BG_PALETTE.index(1 + 16 * 14).write(Color::DIM_CYAN);
  BG_PALETTE.index(1 + 16 * 15).write(Color::DIM_WHITE);

  //
  BG0CNT.write(BgControl::new().with_screenblock(8));
  for (i, (va, b)) in text_screenblock::<8>()
    .iter()
    .zip(b"Brights!AndDims!".iter().copied())
    .enumerate()
  {
    va.write(
      TextScreenEntry::new().with_tile_id(b as u16).with_palbank(i as u16),
    );
  }
  BG1CNT.write(BgControl::new().with_screenblock(9));
  for va in text_screenblock::<9>().iter() {
    const C: u16 = 16 * 13 + 11;
    va.write(TextScreenEntry::new().with_tile_id(C).with_palbank(7));
  }

  //
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);

  // remove forced_blank, which will begin the display cycle.
  DISPCNT.write(DisplayControl::new().with_display_bg0(true));

  let mut x_off = 0_u16;
  let mut y_off = 0_u16;

  let mut last_k = Keys::default();

  // primary loop
  loop {
    // the user input
    let k = get_keys();

    // update world state
    if k.right() {
      x_off = x_off.wrapping_sub(1);
    } else if k.left() {
      x_off = x_off.wrapping_add(1);
    }
    if k.down() {
      y_off = y_off.wrapping_sub(1);
    } else if k.up() {
      y_off = y_off.wrapping_add(1);
    }
    if k.l() & !last_k.l() {
      y_off = y_off.wrapping_add(8);
    }
    if k.r() & !last_k.r() {
      y_off = y_off.wrapping_sub(8);
    }
    last_k = k;

    /*
    Notes:
    Each screenblock is 32x32 in tiles.
    The display itself is 30x20 in tiles (240x160 px)
    The screenblock will wrap around
    So we can do a rolling output in a single screenblock.
    */

    // wait for v_blank to begin
    VBlankIntrWait();

    // Update the display
    BG0_X.write(x_off);
    BG0_Y.write(y_off);
  }
}
