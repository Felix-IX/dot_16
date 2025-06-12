mod renderer;
mod screen;

use crate::screen::Screen;

fn main() {
    let game = game_runtime::game::Game::new("./examples/ppg-1.p8.png").unwrap();
    let memory = game.runtime().memory(); // Rc<RefCell<Memory>>
    let memory_ref = memory.borrow(); // Ref<Memory>
    let vram = memory_ref.screen(); // &'a [u8], tied to memory_ref

    let console_frontend = Screen::new(vram);
    console_frontend.run();
}
