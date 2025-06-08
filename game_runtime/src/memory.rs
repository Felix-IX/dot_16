use lang::rom_loader::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

pub const MEM_START: usize = 0x0000;
pub const MEM_SIZE: usize = 0x10000;
pub const SPRITE_SHEET_1_START: usize = 0x0000;
pub const SPRITE_SHEET_1_END: usize = 0x0fff; // 0x0000..0x1000
pub const SPRITE_SHEET_2_START: usize = 0x1000;
pub const SPRITE_SHEET_2_END: usize = 0x2000; // shared with map rows 32-63
pub const MAP_ROWS_0_31_START: usize = 0x2000;
pub const MAP_ROWS_0_31_END: usize = 0x3000;

pub const SPRITE_FLAGS_START: usize = 0x3000;
pub const SPRITE_FLAGS_END: usize = 0x3100;

pub const MUSIC_START: usize = 0x3100;
pub const MUSIC_END: usize = 0x3200;

pub const SFX_START: usize = 0x3200;
pub const SFX_END: usize = 0x4300;

pub const WORK_RAM_START: usize = 0x4300;
pub const WORK_RAM_END: usize = 0x5600;

pub const CUSTOM_FONT_START: usize = 0x5600;
pub const CUSTOM_FONT_END: usize = 0x5e00;

pub const CART_DATA_START: usize = 0x5e00;
pub const CART_DATA_END: usize = 0x5f00;

pub const DRAW_STATE_START: usize = 0x5f00;
pub const DRAW_STATE_END: usize = 0x5f40;

pub const HARDWARE_STATE_START: usize = 0x5f40;
pub const HARDWARE_STATE_END: usize = 0x5f80;

pub const GPIO_START: usize = 0x5f80;
pub const GPIO_END: usize = 0x6000;

pub const SCREEN_START: usize = 0x6000;
pub const SCREEN_END: usize = 0x8000;

pub const EXTENDED_RAM_START: usize = 0x8000;
pub const EXTENDED_RAM_END: usize = 0x10000; // 0xffff + 1

pub struct Memory {
    pub mem: Box<[u8; MEM_SIZE]>,
}

impl Memory {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            mem: Box::new([0; MEM_SIZE]),
        }))
    }

    pub fn init(&mut self, cart: Cartridge) {
        let data = cart.data();
        if data.len() > (SFX_END - SPRITE_SHEET_1_START) {
            panic!("Cartridge data too large for memory region");
        }

        self.mem[SPRITE_SHEET_1_START..SFX_END].copy_from_slice(cart.data());
    }

    pub fn read(&self, addr: usize) -> u8 {
        assert!(addr < MEM_SIZE, "Invalid memory address: {}", addr);
        self.mem[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        assert!(addr < MEM_SIZE, "Invalid memory address: {}", addr);
        self.mem[addr] = val;
    }

    pub fn set(&mut self, addr: usize, val: u8, len: usize) {
        assert!(addr + len <= MEM_SIZE, "Memory out of bounds: {} + {} > {}", addr, len, MEM_SIZE);
        let temp = vec![val; len];
        self.mem[addr..addr + len].copy_from_slice(&temp);       
    }

    pub fn copy(&mut self, dest: usize, src: usize, len: usize) {
        assert!(src + len <= MEM_SIZE,
                "Memory from source out of bounds: {} + {} > {}", src, len, MEM_SIZE);
        assert!(dest + len <= MEM_SIZE,
                "Memory to destination out of bounds: {} + {} > {}", dest, len, MEM_SIZE);
        let temp = self.mem[src..src + len].to_vec();
        self.mem[dest..dest + len].copy_from_slice(&temp);
    }

    // These are the read-only getters
    pub fn read_sprite(&self, sprite: usize) -> &[u8] {
        let addr = 512 * (sprite / 16) + 4 * (sprite % 16);
        &self.mem[addr..addr + 64]
    }

    pub fn read_map_title(&self, x: usize, y: usize) -> u8 {
        // The map is 128 * 64 (8192 bytes)
        let offset = y * 128 + x;
        if y < 32 {
            // The first 32 rows begin at 0x2000
            self.mem[MAP_ROWS_0_31_START + offset]
        } else {
            // last 32 rows are in shared area with the sprite sheet
            self.mem[SPRITE_SHEET_2_START + offset]
        }
    }

    pub fn sprite_flags(&self) -> &[u8] {
        &self.mem[0x3000..=0x30ff]
    }

    pub fn music(&self) -> &[u8] {
        &self.mem[MUSIC_START..=MUSIC_END]
    }

    pub fn sound_effects(&self) -> &[u8] {
        &self.mem[SFX_START..=SFX_END]
    }

    pub fn work_ram(&self) -> &[u8] {
        &self.mem[WORK_RAM_START..=WORK_RAM_END]
    }

    pub fn font_ram(&self) -> &[u8] {
        &self.mem[CUSTOM_FONT_START..=CUSTOM_FONT_END]
    }

    pub fn cartridge_data(&self) -> &[u8] {
        &self.mem[CART_DATA_START..=CART_DATA_END]
    }

    pub fn draw_state(&self) -> &[u8] {
        &self.mem[DRAW_STATE_START..=DRAW_STATE_END]
    }

    pub fn gpio_pins(&self) -> &[u8] {
        &self.mem[GPIO_START..=GPIO_END]
    }

    pub fn screen(&self) -> &[u8] {
        &self.mem[SCREEN_START..=SCREEN_END]
    }

    pub fn extended_map(&self) -> &[u8] {
        &self.mem[EXTENDED_RAM_START..=EXTENDED_RAM_END]
    }
}
