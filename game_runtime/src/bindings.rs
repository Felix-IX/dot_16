use crate::memory::Memory;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_pico8_apis(lua: &Lua, memory: Rc<RefCell<Memory>>) -> Result<()> {
    let globals = &lua.globals();

    let memory_clone_for_poke = memory.clone();
    let memory_clone_for_peek = memory.clone();
    let memory_clone_for_memset = memory.clone();
    let memory_clone_for_memcpy = memory.clone();

    // poke(addr, val)
    {
        let poke_fn = lua.create_function(move |_, (addr, val): (usize, u8)| {
            memory_clone_for_poke.borrow_mut().write(addr, val);

            Ok(())
        })?;

        globals.set("poke", poke_fn)?;
    }

    // peek(addr)
    {
        let peek_fn = lua.create_function(move |_, addr: usize| {
            let val = memory_clone_for_peek.borrow_mut().read(addr);

            Ok(val)
        })?;

        globals.set("peek", peek_fn)?;
    }

    // memset(dest, val, len)
    {
        let memset_fn = lua.
            create_function(move |_, (dest, val, len): (usize, u8, usize)| {
                memory_clone_for_memset.borrow_mut().set(dest, val, len);

                Ok(())
            })?;
        
        globals.set("memset", memset_fn)?;
    }
    
    // memcpy(dest, src, len)
    {
        let memcpy_fn = lua
            .create_function(move |_, (dest, src, len): (usize, usize, usize)| {
                memory_clone_for_memcpy.borrow_mut().copy(dest, src, len);
                
                Ok(())
            })?;
        globals.set("memcpy", memcpy_fn)?;   
    }    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::*;
    use mlua::Lua;

    #[test]
    fn test_peek_and_poke() {
        let lua = Lua::new();

        // Allocate memory space
        let memory = Rc::new(RefCell::new(Memory {
            mem: Box::new([0; MEM_SIZE]),
        }));

        // Register API
        register_pico8_apis(&lua, memory.clone()).expect("Failed to register APIs");

        // Run poke and peek in Lua
        lua.load(
            r#"
            poke(0x1000, 123)
        "#,
        )
        .exec()
        .expect("Lua exec failed");

        // Assert that the value was written to memory
        assert_eq!(memory.borrow().mem[0x1000], 123);

        // Read the value back from memory
        let val: u8 = lua
            .load("return peek(0x1000)")
            .eval()
            .expect("Lua eval failed");
        assert_eq!(val, 123);
    }
    
    #[test]
    fn test_memset_and_memcpy() {
        let lua = Lua::new();
        
        // Allocate memory space
        let memory = Rc::new(RefCell::new(Memory {
            mem: Box::new([0; MEM_SIZE]),
        }));
        
        // Register API
        register_pico8_apis(&lua, memory.clone()).expect("Failed to register APIs");
        
        // Run memset and memcpy in Lua
        lua.load(
            r#"
            memset(0x1000, 123, 10)
            memcpy(0x2000, 0x1000, 10)"#
        ).exec().expect("Lua exec failed");
        
        // Assert that the values were written to memory
        assert_eq!(memory.borrow().mem[0x1000..0x100a], [123; 0x000a]); // 10 bytes
        assert_eq!(memory.borrow().mem[0x2000..0x200a], [123; 0x000a]);
    }
}
