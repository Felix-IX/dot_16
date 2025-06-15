use crate::bindings::register_pico8_apis;
use crate::runtime::Runtime;
use lang::rom_loader::Cartridge;
use mlua::{Function, Value};
use std::error::Error;
use std::path::PathBuf;

pub struct Game {
    runtime: Runtime,
    cartridge: Cartridge,
    init_fn: Option<Function>,
    update_fn: Option<Function>,
    draw_fn: Option<Function>,
}

impl Game {
    pub fn new(path: PathBuf, path_to_preprocessor: PathBuf) -> Result<Self, Box<dyn Error>> {
        let runtime = Runtime::new()?;
        let cartridge = Cartridge::new(path, path_to_preprocessor)?;
        runtime
            .memory()
            .borrow_mut()
            .init(&cartridge);

        Ok(Game {
            runtime,
            cartridge,
            init_fn: None,
            update_fn: None,
            draw_fn: None,
        })
    }

    // Getters
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn init(&mut self) {
        self.runtime.init(&self.cartridge);
    }

    pub fn run(&mut self) {
        let code = self.cartridge.code();

        let globals = self.runtime().lua_vm().globals();

        for pair in globals.clone().pairs::<String, Value>() {
            match pair {
                Ok((k, v)) => {
                    println!("Lua global: {} => {:?}", k, v);
                }
                Err(e) => println!("Error reading global: {:?}", e),
            }
        }

        register_pico8_apis(&self, &globals).expect("Failed to register Lua APIs");

        self.runtime
            .lua_vm()
            .load(code)
            .exec()
            .expect("Failed to run game");

        match globals.get::<Function>("_init") {
            Ok(fn_) => self.init_fn = Some(fn_),
            Err(_) => {}
        }

        match globals.get::<Function>("_update") {
            Ok(fn_) => self.update_fn = Some(fn_),
            Err(_) => {}
        }

        match globals.get::<Function>("_draw") {
            Ok(fn_) => self.draw_fn = Some(fn_),
            Err(_) => {}
        }
    }
}

/*#[cfg(tests)]
mod tests {
    use lang::rom_loader::resolve_path_from_root;
    use super::*;

    #[tests]
    fn test_new() {
        let path = resolve_path_from_root("../examples/ppg-1.p8.png");
        let _ = Game::new(path).expect("Failed to load game");
    }

    #[tests]
    fn test_init() {
        let mut game = Game::new("../examples/ppg-1.p8.png").expect("Failed to load game");

        game.init();

        game.run();

        assert!(game.init_fn.is_some());
        // assert!(game.update_fn.is_some()); // Since not every game has an update function
        assert!(game.draw_fn.is_some());

        game.init();
    }
}*/
