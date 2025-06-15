use env_logger;
use game_runtime;
use graphics::screen::Screen;
use utils::path_resolver::path_from_workspace_root;
use log::info;

fn main() {
    env_logger::init();
    info!("Setting up");

    let path = path_from_workspace_root("assets/examples/ppg-1.p8.png");
    let path_to_preprocessor = path_from_workspace_root("lang/pico8_patcher/pico8-to-lua.lua");
    let game = game_runtime::game::Game::new(path, path_to_preprocessor).unwrap();
    let memory = game.runtime().memory(); // Rc<RefCell<Memory>>
    let memory_ref = memory.borrow(); // Ref<Memory>
    let vram = memory_ref.screen(); // &'a [u8], tied to memory_ref
    let sprite_sheet = memory_ref.read_sprite_sheet();

    let console_frontend = Screen::new(sprite_sheet);
    println!("{:?}", memory_ref.read_sprite(9));
    console_frontend.run();
}
