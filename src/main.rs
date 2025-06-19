use env_logger;
use game_runtime;
use graphics::screen::Screen;
use log::info;
use utils::path_resolver::path_from_workspace_root;

fn main() {
    env_logger::init();
    info!("Setting up");

    let path = path_from_workspace_root("assets/examples/ppg-1.p8.png");
    let path_to_preprocessor = path_from_workspace_root("lang/pico8_patcher/pico8-to-lua.lua");
    let mut game = game_runtime::game::Game::new(path, path_to_preprocessor).unwrap();

    game.run();
    
    let memory = game.runtime().memory(); // Rc<RefCell<Memory>>
    let memory_ref = memory.borrow(); // RefMut<Memory>
    let vram = memory_ref.screen(); // &'a [u8], tied to memory_ref

    let console_frontend = Screen::new(vram);
    console_frontend.run();
}

// When you need to run something in the machine, copy this after the run function
/*
game
        .runtime()
        .lua_vm()
        .load(
            r#"
            spr(1, 10, 10, 0, 0, false, false)"#,
        )
        .exec().expect("test panic");
*/
