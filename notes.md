
VRAM is 96kb overall, split into 6 "Charblock" regions (16kb each).

Tiles:
* 8x8 px
* 4bpp or 8bpp
* Always aligned to 4
* Vram doesn't even behave with 1-byte writes.
* So we model tiles as `T4 = [u32;8]` or `T8 = [u32;16]`
* A charblock has 524 x T4 or 256 x T8, depending on how you view the memory

Tile indexes vary based on if they're for the bg or obj.
* BG tiles use 4bpp or 8bpp indexing depending on their mode.
* OBJ tiles are are always a 4bpp index (even for an 8bpp OBJ).
* The BG can index up to 1024 either way, so you can index from one charblock into the next, but you cannot access past the end of the BG charblock.

The screenblocks indexes are every 2kb.
* Text screenblocks are 2 bytes per entry, and always 32x32 entries big.
  * Depending on background settings, a text background can use 1, 2, or 4 screenblocks in a row.
* Affine screenblocks have 1 byte per entry, giving just a tile ID.
  * depending on background settings, an affine background is 16x16 tiles up to 128x128 tiles.
