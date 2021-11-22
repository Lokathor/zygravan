#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::mem::size_of_val;

use zygravan::gba::{
  charblock4, get_keys, place_cp437_data, text_screenblock, BgControl,
  BitUnPack, Color, DisplayControl, DisplayStatus, HuffUnCompReadNormal,
  IrqBits, LZ77UnCompReadNormalWrite16bit, TextScreenEntry, UnPackInfo,
  VBlankIntrWait, VideoMode::VideoMode3, BACKDROP, BG0CNT, BG0_X, BG0_Y,
  BG1CNT, BG_PALETTE, DISPCNT, DISPSTAT, IE, IME,
};

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
  // system setup
  let pink = Color::from_rgb(28, 15, 15);
  for x in 0..16 {
    BG_PALETTE.index(0 + 16 * x).write(pink);
  }

  place_cp437_data(charblock4::<0>());
  /*
  // this copies the data into the second half of
  // the same charblock with each pixel that was a 1 being a 2 instead.
  // theoretically we could have 7 sets of tiles
  // (and a little space for screenblocks)
  // and each tile set could have its own possible 16 colors.
  // but... probably we don't want to do any of that.
  let cb = charblock4::<0>();
  for i in 0..256 {
    let mut tile4 = cb.index(i).read();
    for u in tile4.iter_mut() {
      let mut mask = 0b1111;
      let mut extr = 0b0001;
      while mask != 0 {
        if *u & mask != 0 {
          *u += extr;
        }
        mask <<= 4;
        extr <<= 4;
      }
    }
    cb.index(i + 256).write(tile4);
  }
  */
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

  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);

  // remove forced_blank, which will begin the display cycle.
  DISPCNT.write(DisplayControl::new().with_display_bg0(true));

  let mut x_off = 0_u16;
  let mut y_off = 0_u16;

  // primary loop
  loop {
    // the user input
    let k = get_keys();

    // update world state
    if k.right() {
      x_off = x_off.wrapping_sub(1);
    }
    if k.left() {
      x_off = x_off.wrapping_add(1);
    }
    if k.down() {
      y_off = y_off.wrapping_sub(1);
    }
    if k.up() {
      y_off = y_off.wrapping_add(1);
    }

    // wait for v_blank to begin
    VBlankIntrWait();

    // Update the display
    BG0_X.write(x_off);
    BG0_Y.write(y_off);
  }
}
