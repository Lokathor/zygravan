#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::mem::size_of_val;

use zygravan::gba::{
  charblock, get_keys, place_cp437_data, text_screenblock, BgControl,
  BitUnPack, Color, DisplayControl, DisplayStatus, HuffUnCompReadNormal,
  IrqBits, LZ77UnCompReadNormalWrite16bit, TextScreenEntry, UnPackInfo,
  VBlankIntrWait, VideoMode::VideoMode3, BACKDROP, BG0CNT, BG_PALETTE, DISPCNT,
  DISPSTAT, IE, IME,
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
  BACKDROP.write(Color::MAGENTA);

  place_cp437_data(charblock::<0>());
  BG_PALETTE.index(1 + 16 * 0).write(Color::BLACK);
  BG_PALETTE.index(1 + 16 * 1).write(Color::RED);
  BG_PALETTE.index(1 + 16 * 2).write(Color::GREEN);
  BG_PALETTE.index(1 + 16 * 3).write(Color::YELLOW);
  BG_PALETTE.index(1 + 16 * 4).write(Color::BLUE);
  BG_PALETTE.index(1 + 16 * 5).write(Color::MAGENTA);
  BG_PALETTE.index(1 + 16 * 6).write(Color::CYAN);
  BG_PALETTE.index(1 + 16 * 7).write(Color::WHITE);
  //
  BG_PALETTE.index(2 + 16 * 0).write(Color::DIM_BLACK);
  BG_PALETTE.index(2 + 16 * 1).write(Color::DIM_RED);
  BG_PALETTE.index(2 + 16 * 2).write(Color::DIM_GREEN);
  BG_PALETTE.index(2 + 16 * 3).write(Color::DIM_YELLOW);
  BG_PALETTE.index(2 + 16 * 4).write(Color::DIM_BLUE);
  BG_PALETTE.index(2 + 16 * 5).write(Color::DIM_MAGENTA);
  BG_PALETTE.index(2 + 16 * 6).write(Color::DIM_CYAN);
  BG_PALETTE.index(2 + 16 * 7).write(Color::DIM_WHITE);

  BG0CNT.write(BgControl::new().with_screenblock(8));
  for (va, b) in
    text_screenblock::<8>().iter().zip(b"Hello, world.".iter().copied())
  {
    va.write(TextScreenEntry::new().with_tile_id(b as u16).with_palbank(7));
  }

  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);

  // remove forced_blank, which will begin the display cycle.
  DISPCNT.write(DisplayControl::new().with_display_bg0(true));

  const BASE_COLOR: Color = Color::from_rgb(15, 15, 0);

  // primary loop
  loop {
    // the user input
    let k = get_keys();

    // update world state
    let color = Color::from_bits(BASE_COLOR.to_bits() | u16::from(k));

    // wait for v_blank to begin
    VBlankIntrWait();

    // Update the display
    //BG_PALETTE.index(1).write(color);
  }
}
