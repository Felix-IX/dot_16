use crate::memory::Memory;
use lang::rom_loader::Cartridge;
use mlua::Lua;

pub fn init() {
    let cartridge = Cartridge::new("../examples/ppg-1.p8.png").expect("Could not load cartridge");

    let mem = Memory::new();

    mem.borrow_mut().init(cartridge);
}

pub fn run(code: &Vec<u8>) {
    let lua_vm = Lua::new();
    lua_vm.load(code).exec().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let cartridge = Cartridge::new("../examples/ppg-1.p8.png").unwrap();
        let mut memory = Memory::new();
        memory.borrow_mut().init(cartridge)
    }
}
