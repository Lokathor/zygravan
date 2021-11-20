#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::mem::size_of_val;

use zygravan::gba::{
  get_keys, BitUnPack, Color, DisplayControl, DisplayStatus,
  HuffUnCompReadNormal, IrqBits, LZ77UnCompReadNormalWrite16bit, UnPackInfo,
  VBlankIntrWait, VideoMode::VideoMode3, BACKDROP, DISPCNT, DISPSTAT, IE, IME,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  BACKDROP.write(Color::from_rgb(31, 0, 0));
  loop {}
}

/*
#[allow(dead_code)]
extern "C" fn irq_handler(_bits: IrqBits) {
  //
}
*/

#[no_mangle]
extern "C" fn main() -> ! {
  // system setup

  // unpack the tile data
  unsafe {
    BACKDROP.add(1).write(Color::from_rgb(31, 31, 31));
    LZ77UnCompReadNormalWrite16bit(CP437.as_ptr(), 0x06000000 as _);
  }

  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);

  // remove forced_blank, which will begin the display cycle.
  DISPCNT.write(DisplayControl::new());

  // primary loop
  loop {
    // the user input
    let k = get_keys();

    // update world state
    // (TODO)

    // wait for v_blank to begin
    VBlankIntrWait();

    // Update the display
    BACKDROP.write(Color::from_bits(k.into()));
  }
}

/// Code Page 437 tiles.
///
/// * Data is lz77 compressed, decompress with
///   [`LZ77UnCompReadNormalWrite16bit`]
const CP437: &[u32] = &[
  0x00200010, 0xF0000030, 0x10059001, 0x42011111, 0x01060001, 0x20101001,
  0x111E1107, 0x01100110, 0x1B101A00, 0x73111F10, 0x10010011, 0x00081019,
  0x10153000, 0x1F109F12, 0x10001101, 0x03202320, 0x21004F00, 0x505110FF,
  0x00730060, 0xF017100F, 0x101B701F, 0x4760FF33, 0x37408A10, 0x43F04720,
  0xC8701360, 0x30FFC500, 0x500B309D, 0x60B5305E, 0x30BF9079, 0xCF2420CE,
  0xF6000001, 0x03300000, 0x1A60FE10, 0x21FFF330, 0x21053113, 0x4107211F,
  0x1145110E, 0xDF4B0116, 0x01701700, 0x50071110, 0x5003405B, 0xFE77119F,
  0x3240B340, 0x5D010720, 0x46414D50, 0xFF007A21, 0x07306B11, 0x79510320,
  0x9B11F730, 0xAD31FB10, 0x018210FF, 0x01981193, 0x302111D1, 0x015F5186,
  0xF001FF67, 0x38113981, 0xC3217C11, 0x58312820, 0x41FF5C31, 0x50AF6057,
  0x80C750BB, 0x21E360DB, 0xFF07408E, 0x59023B52, 0x68110F01, 0xA0310370,
  0x23226D21, 0x121650FF, 0xF23F510B, 0x41E540AF, 0x909FC0FC, 0xAE22FF9B,
  0x0370BFF0, 0x17803752, 0x5BA2DB90, 0x12FFA231, 0x221BB064, 0x207E52C3,
  0xD0C321CB, 0xEFDA4101, 0xBB50E231, 0x22108161, 0x1166337F, 0xFF3C53B4,
  0x8B63DBB2, 0xE3C2B752, 0x01F0BBF0, 0x031498F0, 0x309F13FF, 0x40BB60DA,
  0xE1AED1C3, 0x30D2418F, 0x0770FFFB, 0x1D232F60, 0xCF31DA32, 0x6B402F14,
  0x24FD3E61, 0x32C34345, 0x43B8313B, 0x101342F7, 0x43FF4432, 0x6067525B,
  0x6401F036, 0x20F6C117, 0xFF3E728C, 0x4C800F60, 0x0F34FEB1, 0x2B44FE24,
  0xCA33DCF1, 0xF043A1FF, 0xE20CA201, 0xF0D6B146, 0xD04CD201, 0x4271FFE0,
  0x7B31DF63, 0x8033A525, 0xC024C815, 0xD2FF9F91, 0x148B70A0, 0x549864C3,
  0xF0AE44B2, 0xFF27431F, 0xCC44BB93, 0x4393E131, 0xF664BC35, 0x3FE01B42,
  0x447360FF, 0x555FC0C3, 0x82FCA03E, 0x55BF50C7, 0x3FD0FF8F, 0x98711FB0,
  0x6FF11321, 0x0FF00FF0, 0xC1FFDFB1, 0xF17FD297, 0xB207B2FB, 0xE1E7B1AF,
  0xFF08937F, 0xFF41FF60, 0x03507C15, 0xFF738B63, 0x8E56B766, 0x35C442FF,
  0x86C14642, 0x478264CB, 0xB7DF943B, 0xD755FF3F, 0x03440597, 0xF1240B45,
  0x87248508, 0x06FF0710, 0xF02355B6, 0x6736571F, 0x377FC0E0, 0xFF6050AA,
  0xDB908F97, 0xF7A4DF50, 0xBF82A485, 0x7FE254A2, 0x30D380FF, 0x470F41BF,
  0x868E5807, 0x403F517A, 0x8F13FF9F, 0xFB183F79, 0x1F702457, 0x44774097,
  0x97FF1F95, 0xF1435758, 0x910E37BF, 0x18FBA13F, 0xFFD908BB, 0x0043B728,
  0xDFB03FF0, 0x57951F54, 0xD0099FD3, 0x535FF1FF, 0xA401F0BF, 0x5919F05F,
  0x483B9657, 0x3B11FF54, 0x5111AF1A, 0x13913F91, 0x1FD22F81, 0x84FF0397,
  0x1848295F, 0xA19F46A2, 0xE8B332BF, 0xFFDF8046, 0xA04AC3F3, 0x3CF03F61,
  0x3F9B8F47, 0x01F0B380, 0x5A01F0FF, 0xF09483FB, 0x3807AC20, 0x77AB37E3,
  0x9EA0FF5F, 0x20A5FF93, 0x1F366795, 0xD84AFFA5, 0x97FF5B50, 0xA83FE0BF,
  0x5BFF772B, 0x7CBFD2DE, 0xFF6166C7, 0xEB5AC765, 0x3F28D84C, 0xC357BF64,
  0x9FF20763, 0xF408A9FF, 0x6A1B3703, 0x7303B452, 0xB05BF41F, 0x425AFF5F,
  0x0344478D, 0xA7C3E3B9, 0xC7D364C8, 0x13FF5FE0, 0x9A07C4D7, 0xB03FF145,
  0x7B1734A4, 0xFF7FF1E1, 0xFF7DE3F9, 0x047B1F30, 0x42E91C0E, 0xFFC0E3D3,
  0xCBA7F3FF, 0x7DFF8382, 0x6A63AA1D, 0xC04BF347, 0x3F72FF5F, 0xF8309397,
  0xAB79DB2C, 0xFEA8092E, 0x6BFF5F4A, 0x806FFC40, 0x9933E00F, 0xC397514A,
  0xFFBCF36B, 0x9F9407F6, 0xCF4C97E3, 0x03F18395, 0x9FF32069, 0x2C3F88FF,
  0x4E1C3D93, 0x2F7F678C, 0xA467A75B, 0xFF49FF5F, 0x1FF06697, 0x1FF02FC3,
  0x7A51FFB2, 0x2CFF844D, 0x489F7033, 0x70A34DC3, 0x86DFF0F3, 0xFF67C99F,
  0x47681FE0, 0x7F99DFF3, 0xC07848B5, 0x1FD726F5, 0x673B30FF, 0x7D8F46E3,
  0x7BEBFA1F, 0x5F877213, 0xAF56FF0A, 0xA71F6BDC, 0x313CFF45, 0xD74D8067,
  0x73FF9F54, 0xFBFFA787, 0xF03B834B, 0xD11FB01F, 0xFF9FFB63, 0x438F9FF2,
  0x1FF0CA7A, 0x1FC6DFF2, 0x1F626241, 0x66BB5AFF, 0xBC7BF01E, 0x3BAF57DF,
  0x88E73E57, 0x137AFE14, 0xCF39BF66, 0x324393D0, 0xF7A547CE, 0x7F017F11,
  0xF7D1466D, 0x173F3820, 0x837B26BE, 0x3FF3FFBF, 0x7FF28073, 0xBFF1009F,
  0x7FF11FF0, 0xF6FFBB96, 0x6B5F7AC3, 0x5FD940A7, 0x3ADF7BE3, 0xFF345CDD,
  0x736BFBA3, 0xDB6EF37A, 0x1E7EBEC7, 0x8BAE1FF7, 0x8F61F5FF, 0xAE21B09F,
  0x4C2C4FDD, 0x80D15FE1, 0xBA11FF1F, 0xE510D125, 0x1BF61449, 0xC44C13A0,
  0x35FF5350, 0x90A2B05B, 0x407B7017, 0x0D4B0DC0, 0x9107F04E, 0x10100F40,
  0x01013407, 0xFF07F001, 0x763C0F30, 0x5A17D102, 0x0FC0A91C, 0x03F0BBFA,
  0x2FF396FF, 0x6017F0E7, 0x1A1FE017, 0x1D038087, 0x13A0FF07, 0x1FF0A3F1,
  0x5FF08352, 0x57205F60, 0xC0FF3727, 0xF003F05F, 0xF077505F, 0x87B7B03F,
  0xFF988997, 0xB8C927C0, 0x99F21FF1, 0x5FF127CA, 0x5FF0C4CB, 0xD99F81FF,
  0xF053FCD9, 0xF224EC53, 0xF0939CC3, 0x7FB0FF7F, 0x578057F0, 0x6BF11FC0,
  0x03F08123, 0xF6FF5424, 0xF0FCE3F8, 0x3877423F, 0xA03FF0CF, 0xFF2720D7,
  0x7F803FF0, 0x3FE01FC0, 0x7FE098C7, 0xB7C15FC0, 0xE23FF0FF, 0xD0BFF05F,
  0xCEBFF17F, 0x70DFF053, 0xDFF1FF67, 0x7FF0BFE1, 0x7FF0DFF1, 0x77F0A485,
  0xF1FFDFA0, 0xF2D7601F, 0xF087F45F, 0x93A7F09C, 0xFF01F02F, 0xBEF00770,
  0x03F021F0, 0x11F00770, 0x4DF003B0, 0x9C01F0FF, 0x6DC50C47, 0x907BFFCB,
  0x6FF98E07, 0xC6BEFFB7, 0xA3577395, 0x3B900790, 0x8347724C, 0xCEFF473B,
  0x716B163F, 0x6F9B9EB6, 0xC0BFCEB5, 0xFF3FF7DB, 0xBBE84B79, 0x5BFB6769,
  0x57DD4B98, 0xFB268B58, 0x3AFF9DFF, 0xB01FF903, 0xBC4727DF, 0xD0543EBB,
  0xDF60FF1F, 0x3F69AF5B, 0x1FBBBE5D, 0xA38801F0, 0xB9FFCA6B, 0x9A5F9D73,
  0x51979973, 0x5F3C59B7, 0xFFFF7E35, 0x9C601780, 0xA0896695, 0x03F06FF3,
  0x93F003D2, 0x9F80E2FF, 0xFA2FEF23, 0xD601F01F, 0xB701F08B, 0x11C5FF8F,
  0x9D5D1593, 0x6F3F433C, 0x3ACB17F3, 0xFAF84381, 0xA4669097, 0xF001F018,
  0x00000001,
];
