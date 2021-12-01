#![no_std]
#![no_main]
#![allow(unused)]
#![deny(unsafe_code)]

use core::{fmt::Write, mem::size_of_val};

use bytemuck::{cast_slice, cast_slice_mut};
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
  chars: [u32; (32 * 32) / 4],
  banks: [u32; (32 * 32) / 4],
  position: u32,
}
impl TerminalPanel {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      chars: [0_u32; (32 * 32) / 4],
      banks: [0_u32; (32 * 32) / 4],
      position: 0,
    }
  }
  #[inline]
  pub fn set_all_chars(&mut self, ch: char) {
    let b = ch as u8;
    let u = u32::from_ne_bytes([b, b, b, b]);
    self.chars.iter_mut().for_each(|ch| *ch = u);
  }
  #[inline]
  pub fn set_all_banks(&mut self, b: u8) {
    let u = u32::from_ne_bytes([b, b, b, b]);
    self.banks.iter_mut().for_each(|bank| *bank = u);
  }
  #[inline]
  pub fn change_line(&mut self, delta: i32) {
    self.position =
      ((self.position >> 5).wrapping_add(delta as u32) << 5) % (32 * 32);
  }
  #[inline]
  pub fn push_bytes(&mut self, bytes: &[u8]) {
    for byte in bytes.iter().copied() {
      match byte {
        b'\n' => self.change_line(1),
        other => {
          let bytes: &mut [u8] = cast_slice_mut::<u32, u8>(&mut self.chars);
          bytes[self.position as usize] = other;
          self.position += 1;
          if (self.position % 32) == 30 {
            self.change_line(1);
          }
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn scroll_point(&self) -> u16 {
    0_u16.wrapping_sub((19 - (self.position >> 5) as u16) * 8)
  }
}
impl core::fmt::Write for TerminalPanel {
  #[inline]
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.push_bytes(s.as_bytes());
    Ok(())
  }
  #[inline]
  fn write_char(&mut self, c: char) -> core::fmt::Result {
    self.push_bytes(core::slice::from_ref(&(c as u8)));
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

  // TODO: put some stuff into OBJ tile memory.

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

  //
  BG0CNT.write(BgControl::new().with_screenblock(8));

  let mut panel = TerminalPanel::new();
  //panel.set_all_chars('.');
  writeln!(panel, "Hello World!").unwrap();
  writeln!(panel, "Another line of text.").unwrap();
  writeln!(panel, "This line of text is in excess of thirty characters.")
    .unwrap();
  write!(panel, ">").unwrap();

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
    /*
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
    */
    last_k = k;

    if (VBLANK_COUNTER.read() % 64) == 1 {
      //x_off = x_off.wrapping_sub(1);
    }

    y_off = panel.scroll_point();

    // wait for v_blank to begin
    VBlankIntrWait();

    // Update the display
    BG0_X.write(x_off);
    BG0_Y.write(y_off);
    TextScreenblock::_8
      .as_volblock()
      .iter()
      .zip(
        cast_slice::<u32, u8>(&panel.chars)
          .iter()
          .copied()
          .zip(cast_slice::<u32, u8>(&panel.banks).iter().copied()),
      )
      .for_each(|(va, (ch, p))| {
        va.write(TextScreenEntry::from_id_bank(ch as u16, p as u16))
      });
  }
}
