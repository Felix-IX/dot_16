use crate::memory::Memory;
use lang::rom_loader::Cartridge;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Runtime {
    lua_vm: Lua,
    memory: Rc<RefCell<Memory>>,
}

impl Runtime {
    pub fn new() -> Result<Self, mlua::Error> {
        let lua_vm = Lua::new();
        let memory = Memory::new();

        Ok(Runtime { lua_vm, memory })
    }

    pub fn init(&self, cart: &Cartridge) {
        self.memory.borrow_mut().init(cart);
    }

    // Getters
    pub fn lua_vm(&self) -> &Lua {
        &self.lua_vm
    }

    pub fn memory(&self) -> &Rc<RefCell<Memory>> {
        &self.memory
    }
}
