use mlua::{Result, Table};
use crate::game::Game;

pub fn register_pico8_apis(game: &Game, global_table: &Table) -> Result<()> {
    let memory_clone_for_poke = game.runtime().memory().clone();
    let memory_clone_for_peek = game.runtime().memory().clone();
    let memory_clone_for_memset = game.runtime().memory().clone();
    let memory_clone_for_memcpy = game.runtime().memory().clone();

    // poke(addr, val)
    {
        let poke_fn = game.runtime().lua_vm().create_function(move |_, (addr, val): (usize, u8)| {
            memory_clone_for_poke.borrow_mut().write(addr, val);

            Ok(())
        })?;

        global_table.set("poke", poke_fn)?;
    }

    // peek(addr)
    {
        let peek_fn = game.runtime().lua_vm().create_function(move |_, addr: usize| {
            let val = memory_clone_for_peek.borrow_mut().read(addr);

            Ok(val)
        })?;

        global_table.set("peek", peek_fn)?;
    }

    // memset(dest, val, len)
    {
        let memset_fn = game.runtime().lua_vm().
            create_function(move |_, (dest, val, len): (usize, u8, usize)| {
                memory_clone_for_memset.borrow_mut().set(dest, val, len);

                Ok(())
            })?;

        global_table.set("memset", memset_fn)?;
    }

    // memcpy(dest, src, len)
    {
        let memcpy_fn = game.runtime().lua_vm()
            .create_function(move |_, (dest, src, len): (usize, usize, usize)| {
                memory_clone_for_memcpy.borrow_mut().copy(dest, src, len);

                Ok(())
            })?;
        global_table.set("memcpy", memcpy_fn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_and_poke() {
        let mut game = Game::new("../examples/ppg-1.p8.png")
            .expect("Failed to load game");

        game.init();
        
        let globals = game.runtime().lua_vm().globals();

        // Register API
        register_pico8_apis(&game, &globals).expect("Failed to register APIs");

        // Run poke and peek in Lua
        game.runtime().lua_vm().load(
            r#"
            poke(0x1000, 123)
        "#,
        )
        .exec()
        .expect("Lua exec failed");

        // Assert that the value was written to memory
        assert_eq!(game.runtime().memory().borrow().mem[0x1000], 123);

        // Read the value back from memory
        let val: u8 = game.runtime().lua_vm()
            .load("return peek(0x1000)")
            .eval()
            .expect("Lua eval failed");
        assert_eq!(val, 123);
    }

    #[test]
    fn test_memset_and_memcpy() {
        let mut game = Game::new("../examples/tictactoe32-0.p8.png")
            .expect("Failed to load game");

        game.init();
        
        let globals = game.runtime().lua_vm().globals();

        // Register API
        register_pico8_apis(&game, &globals).expect("Failed to register APIs");

        // Run memset and memcpy in Lua
        game.runtime().lua_vm().load(
            r#"
            memset(0x1000, 123, 10)
            memcpy(0x2000, 0x1000, 10)"#
        ).exec().expect("Lua exec failed");

        // Assert that the values were written to memory
        assert_eq!(game.runtime().memory().borrow().mem[0x1000..0x100a], [123; 0x000a]); // 10 bytes
        assert_eq!(game.runtime().memory().borrow().mem[0x2000..0x200a], [123; 0x000a]);
    }
}
