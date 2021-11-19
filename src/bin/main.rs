#![no_std]
#![no_main]
#![allow(unused_imports)]

use zygravan::gba::{
  get_keys, Color, DisplayControl, DisplayStatus, IrqBits, VBlankIntrWait,
  VideoMode::VideoMode3, BACKDROP, DISPCNT, DISPSTAT, IE, IME,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  BACKDROP.write(Color::from_rgb(31, 0, 0));
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  // system setup
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);
  DISPCNT.write(DisplayControl::new());
  //panic!();
  BACKDROP.write(Color::from_rgb(0, 0, 31));

  // primary loop
  loop {
    //let _k = get_keys();
    // update world state
    // wait for v_blank to begin
    VBlankIntrWait();
    //BACKDROP.write(Color::from_bits(k.into()));
    BACKDROP.write(Color::from_rgb(0, 31, 0));
  }
}

/*
#[allow(dead_code)]
extern "C" fn irq_handler(_bits: IrqBits) {
  //
}
*/
