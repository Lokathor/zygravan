#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unused_imports)]

use core::{fmt::Write, mem::size_of_val};

use bytemuck::cast_slice_mut;
use zygravan::{gba::*, Ewram};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
  PalRam::backdrop().write(Color::from_rgb(31, 0, 0));
  loop {}
}

static VBLANK_COUNTER: GbaCell<u32> = GbaCell::new_u32(0);

extern "C" fn irq_handler(bits: IrqBits) {
  if bits.vblank() {
    VBLANK_COUNTER.write(VBLANK_COUNTER.read().wrapping_add(1));
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalPanel {
  block: TextScreenblock,
  position: u16,
  palbank: u16,
}
impl TerminalPanel {
  pub const fn from_screenblock(block: TextScreenblock) -> Self {
    Self { block, position: 0, palbank: 0 }
  }
  pub fn set_palbank(&mut self, palbank: u16) {
    self.palbank = palbank;
  }
}
impl core::fmt::Write for TerminalPanel {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    for b in s.as_bytes().iter().copied() {
      match b {
        b'\n' => {
          self.position = ((self.position / 32) + 1) * 32;
        }
        other => {
          self
            .block
            .as_volblock()
            .index(self.position as usize)
            .write(TextScreenEntry::from_id_bank(other as u16, self.palbank));
          self.position += 1;
          if (self.position % 32) == 30 {
            // advance to next row
            self.position += 2;
          }
        }
      }
      //
      if (self.position / 32) == 20 {
        // reset to the first line
        self.position = 0;
      }
    }
    //
    Ok(())
  }
}

#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn main() -> ! {
  //

  if let Some(mut ewram) = Ewram::try_new() {
    let ewram_bytes: &mut [u8] = cast_slice_mut(&mut *ewram);
    let hello_world = b"HelloWorld";
    ewram_bytes.iter_mut().zip(hello_world.iter()).for_each(|(e, b)| *e = *b);
  }

  decompress_cp437_data_to(BgCharblock::_0.tiles4());

  //
  let pink = Color::from_rgb(28, 15, 15);
  for x in 0..16 {
    PalRam::bg_palbank(x).index(0).write(pink);
  }

  //
  PalRam::bg_palbank(0).index(1).write(Color::BLACK);
  PalRam::bg_palbank(1).index(1).write(Color::RED);
  PalRam::bg_palbank(2).index(1).write(Color::GREEN);
  PalRam::bg_palbank(3).index(1).write(Color::YELLOW);
  PalRam::bg_palbank(4).index(1).write(Color::BLUE);
  PalRam::bg_palbank(5).index(1).write(Color::MAGENTA);
  PalRam::bg_palbank(6).index(1).write(Color::CYAN);
  PalRam::bg_palbank(7).index(1).write(Color::WHITE);
  PalRam::bg_palbank(8).index(1).write(Color::DIM_BLACK);
  PalRam::bg_palbank(9).index(1).write(Color::DIM_RED);
  PalRam::bg_palbank(10).index(1).write(Color::DIM_GREEN);
  PalRam::bg_palbank(11).index(1).write(Color::DIM_YELLOW);
  PalRam::bg_palbank(12).index(1).write(Color::DIM_BLUE);
  PalRam::bg_palbank(13).index(1).write(Color::DIM_MAGENTA);
  PalRam::bg_palbank(14).index(1).write(Color::DIM_CYAN);
  PalRam::bg_palbank(15).index(1).write(Color::DIM_WHITE);

  // TODO: the terminal panel should just be some arrays or whatever, but
  // writing to the terminal panel should not directly write to VRAM. Instead,
  // writes to the terminal panel should fill an array which we send to the vram
  // during a vblank. This separation prevents vram modifications during vdraw.

  //
  BG0CNT.write(BgControl::new().with_screenblock(8));

  let mut panel = TerminalPanel::from_screenblock(TextScreenblock::_8);
  for (ch, palbank) in "Brights!AndDims!".chars().zip(0..) {
    panel.set_palbank(palbank);
    write!(panel, "{}", ch).unwrap();
  }
  panel.set_palbank(0);
  writeln!(panel, "\nAnotherLine").unwrap();
  writeln!(panel, ".....|||||-----/////^^^^^%%%%%").unwrap();
  write!(panel, "@").unwrap();
  //
  /*
  BG1CNT.write(BgControl::new().with_screenblock(9));
  let full_block =
    TextScreenEntry::new().with_tile_id(16 * 13 + 11).with_palbank(7);
  TextScreenblock::_9.write_all(full_block);
  // */

  //
  set_irq_handler(Some(irq_handler));
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

    if (VBLANK_COUNTER.read() % 64) == 1 {
      //x_off = x_off.wrapping_sub(1);
    }

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
