#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::mem::size_of_val;

use zygravan::gba::{
  charblock, get_keys, place_cp437_data, screenblock, BgControl, BitUnPack,
  Color, DisplayControl, DisplayStatus, HuffUnCompReadNormal, IrqBits,
  LZ77UnCompReadNormalWrite16bit, TextScreenEntry, UnPackInfo, VBlankIntrWait,
  VideoMode::VideoMode3, BACKDROP, BG0CNT, BG_PALETTE, DISPCNT, DISPSTAT, IE,
  IME,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
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

  place_cp437_data(charblock::<0>());
  BG_PALETTE.index(1).write(Color::from_rgb(31, 31, 31));

  BG0CNT.write(BgControl::new().with_screenblock(8));
  for (va, b) in screenblock::<8>().iter().zip(b"Hello, world.".iter().copied())
  {
    va.write(TextScreenEntry::new().with_tile_id(b as u16));
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
    BG_PALETTE.index(1).write(color);
  }
}
