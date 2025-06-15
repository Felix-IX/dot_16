use crate::renderer::render_4bit_vram;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

const SCREEN_BUFFER_SIZE: usize = 128 * 128;
const SCREEN_SIDE: u32 = 128;

pub struct Screen<'a> {
    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    pixels: Option<Pixels<'a>>,
    vram: &'a [u8],
    frame_count: u64,
}

impl<'a> Screen<'a> {
    pub fn new(vram: &'a [u8]) -> Self {
        Self {
            window: None,
            window_id: None,
            pixels: None,
            frame_count: 0,
            vram,
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop
            .run_app(&mut self)
            .expect("Failed to run event loop");
    }

    fn draw_frame(&mut self) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();

            render_4bit_vram(self.vram, frame);

            if let Err(e) = pixels.render() {
                eprintln!("Pixels render error: {:?}", e);
            }
        };
    }
}

impl<'a> ApplicationHandler for Screen<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Dot-16")
                    .with_resizable(true)
                    .with_inner_size(PhysicalSize::new(1024, 1024)),
            )
            .expect("Failed to create window");

        let window_id = window.id();
        let window_size = window.inner_size();

        let window_arc = Arc::new(window);

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window_arc.clone());
        let pixels = Pixels::new(SCREEN_SIDE, SCREEN_SIDE, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window_arc);
        self.window_id = Some(window_id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if Some(id) != self.window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested, exiting...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw_frame();
                self.frame_count += 1;
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }

            _ => {}
        }
    }
}
