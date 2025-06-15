const VRAM_SIZE: usize = 128 * 128;

pub const PALETTE: [[u8; 4]; 16] = [
    [0x00, 0x00, 0x00, 0xFF], // #000000
    [0x1D, 0x2B, 0x53, 0xFF], // #1D2B53
    [0x7E, 0x25, 0x53, 0xFF], // #7E2553
    [0x00, 0x87, 0x51, 0xFF], // #008751
    [0xAB, 0x52, 0x36, 0xFF], // #AB5236
    [0x5F, 0x57, 0x4F, 0xFF], // #5F574F
    [0xC2, 0xC3, 0xC7, 0xFF], // #C2C3C7
    [0xFF, 0xF1, 0xE8, 0xFF], // #FFF1E8
    [0xFF, 0x00, 0x4D, 0xFF], // #FF004D
    [0xFF, 0xA3, 0x00, 0xFF], // #FFA300
    [0xFF, 0xEC, 0x27, 0xFF], // #FFEC27
    [0x00, 0xE4, 0x36, 0xFF], // #00E436
    [0x29, 0xAD, 0xFF, 0xFF], // #29ADFF
    [0x83, 0x76, 0x9C, 0xFF], // #83769C
    [0xFF, 0x77, 0xA8, 0xFF], // #FF77A8
    [0xFF, 0xCC, 0xAA, 0xFF], // #FFCCAA
];

pub fn render_4bit_vram(
    vram: &[u8],
    frame: &mut [u8], // pixels.get_frame()
) {
    let mut pixel_index = 0;

    for byte in vram {
        let lo = byte >> 4;
        let hi = byte & 0x0f;
        if pixel_index < VRAM_SIZE {
            frame[pixel_index * 4..pixel_index * 4 + 4].copy_from_slice(&PALETTE[lo as usize]);
            pixel_index += 1;
        }
        if pixel_index < VRAM_SIZE {
            frame[pixel_index * 4..pixel_index * 4 + 4].copy_from_slice(&PALETTE[hi as usize]);
            pixel_index += 1;
        }
    }
}

pub fn set_pixel_color(vram: &mut [u8], x: usize, y: usize, col: u8) {
    if x >= 128 || y >= 128 || col > 0x0F {
        return;
    }
    let pixel_index = y * 128 + x; // which pixel
    let byte_index = pixel_index / 2; // which byte
    let is_high = pixel_index % 2 == 0; // even pixel → high byte

    let byte = &mut vram[byte_index];

    if is_high {
        *byte = (*byte & 0x0F) | (col << 4);
    } else {
        *byte = (*byte & 0xF0) | col;
    }
}

pub fn get_pixel_color(vram: &[u8], x: usize, y: usize) -> usize {
    let pixel_index = y * 128 + x; // which pixel
    let byte_index = pixel_index / 2; // which byte
    let is_high = pixel_index % 2 == 0; // even pixel → high byte

    let byte = &vram[byte_index];
    if is_high {
        (*byte >> 4) as usize
    } else {
        *byte as usize
    }
}
