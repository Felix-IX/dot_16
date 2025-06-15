use crate::game::Game;
use graphics::renderer::{get_pixel_color, set_pixel_color};
use mlua::{Result, Table};

pub fn register_pico8_apis(game: &Game, global_table: &Table) -> Result<()> {
    let memory_clone_for_poke = game.runtime().memory().clone();
    let memory_clone_for_peek = game.runtime().memory().clone();
    let memory_clone_for_memset = game.runtime().memory().clone();
    let memory_clone_for_memcpy = game.runtime().memory().clone();
    let memory_clone_for_pset = game.runtime().memory().clone();
    let memory_clone_for_pget = game.runtime().memory().clone();
    let memory_clone_for_spr = game.runtime().memory().clone();

    // poke(addr, val)
    {
        let poke_fn =
            game.runtime()
                .lua_vm()
                .create_function(move |_, (addr, val): (usize, u8)| {
                    memory_clone_for_poke.borrow_mut().write(addr, val);

                    Ok(())
                })?;

        global_table.set("poke", poke_fn)?;
    }

    // peek(addr)
    {
        let peek_fn = game
            .runtime()
            .lua_vm()
            .create_function(move |_, addr: usize| {
                let val = memory_clone_for_peek.borrow_mut().read(addr);

                Ok(val)
            })?;

        global_table.set("peek", peek_fn)?;
    }

    // memset(dest, val, len)
    {
        let memset_fn = game.runtime().lua_vm().create_function(
            move |_, (dest, val, len): (usize, u8, usize)| {
                memory_clone_for_memset.borrow_mut().set(dest, val, len);

                Ok(())
            },
        )?;

        global_table.set("memset", memset_fn)?;
    }

    // memcpy(dest, src, len)
    {
        let memcpy_fn = game.runtime().lua_vm().create_function(
            move |_, (dest, src, len): (usize, usize, usize)| {
                memory_clone_for_memcpy.borrow_mut().copy(dest, src, len);

                Ok(())
            },
        )?;

        global_table.set("memcpy", memcpy_fn)?;
    }

    // pset(x, y, col)
    {
        let pset_fn = game.runtime().lua_vm().create_function(
            move |_, (x, y, col): (usize, usize, u8)| {
                set_pixel_color(memory_clone_for_pset.borrow_mut().screen_mut(), x, y, col);

                Ok(())
            },
        )?;

        global_table.set("pset", pset_fn)?;
    }

    // pget(x, y)
    {
        let pget_fn =
            game.runtime()
                .lua_vm()
                .create_function(move |_, (x, y): (usize, usize)| {
                    let col =
                        get_pixel_color(memory_clone_for_pget.borrow_mut().screen_mut(), x, y);

                    Ok(col)
                })?;

        global_table.set("pget", pget_fn)?;
    }

    // spr(n, x, y, w, h, flip_x, flip_y)
    {
        let spr_fn = game.runtime().lua_vm().create_function(
            move |_,
                  (n, x, y, w, h, flip_x, flip_y): (
                usize,
                usize,
                usize,
                usize,
                usize,
                bool,
                bool,
            )| {
                let sprite = memory_clone_for_spr.borrow().read_sprite(n);

                // to be implemented...

                Ok(())
            },
        )?;

        global_table.set("spr", spr_fn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::path_resolver::path_from_workspace_root;

    #[test]
    fn test_peek_and_poke() {
        unsafe {
            std::env::set_var("WORKSPACE_ROOT", "../..");
        }

        let path = path_from_workspace_root("assets/examples/tictactoe32-0.p8.png");
        let path_to_preprocessor = path_from_workspace_root("lang/pico8_patcher/pico8-to-lua.lua");
        let mut game = Game::new(path, path_to_preprocessor).expect("Failed to load game");
        game.init();

        let globals = game.runtime().lua_vm().globals();

        // Register API
        register_pico8_apis(&game, &globals).expect("Failed to register APIs");

        // Run poke and peek in Lua
        game.runtime()
            .lua_vm()
            .load(
                r#"
            poke(0x1000, 123)
        "#,
            )
            .exec()
            .expect("Lua exec failed");

        // Assert that the value was written to memory
        assert_eq!(game.runtime().memory().borrow().mem[0x1000], 123);

        // Read the value back from memory
        let val: u8 = game
            .runtime()
            .lua_vm()
            .load("return peek(0x1000)")
            .eval()
            .expect("Lua eval failed");
        assert_eq!(val, 123);
    }

    #[test]
    fn test_memset_and_memcpy() {
        let path = path_from_workspace_root("assets/examples/tictactoe32-0.p8.png");
        let path_to_preprocessor = path_from_workspace_root("lang/pico8_patcher/pico8-to-lua.lua");
        let mut game = Game::new(path, path_to_preprocessor).expect("Failed to load game");

        game.init();

        let globals = game.runtime().lua_vm().globals();

        // Register API
        register_pico8_apis(&game, &globals).expect("Failed to register APIs");

        // Run memset and memcpy in Lua
        game.runtime()
            .lua_vm()
            .load(
                r#"
            memset(0x1000, 123, 10)
            memcpy(0x2000, 0x1000, 10)"#,
            )
            .exec()
            .expect("Lua exec failed");

        // Assert that the values were written to memory
        assert_eq!(
            game.runtime().memory().borrow().mem[0x1000..0x100a],
            [123; 0x000a]
        ); // 10 bytes
        assert_eq!(
            game.runtime().memory().borrow().mem[0x2000..0x200a],
            [123; 0x000a]
        );
    }
}
