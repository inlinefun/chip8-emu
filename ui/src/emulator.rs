use std::sync::Arc;

use chip8_core::Chip8;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

#[derive(Default)]
pub(crate) struct Emulator {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    pub(crate) core: Chip8,
}

impl ApplicationHandler for Emulator {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(it) => it,
            Err(e) => panic!("failed to create 'winit' window. {}", e),
        };
        window.set_visible(true);
        let window = Arc::new(window);
        let window_ptr: &'static Window = Box::leak(Box::new(window.clone()));
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window_ptr);
        let pixels = match Pixels::new(64, 32, surface_texture) {
            Ok(it) => it,
            Err(e) => panic!("failed to create a 'pixels' instance. \n{}", e),
        };
        self.window = Some(window);
        self.pixels = Some(pixels);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.core.advance_game_loop();
                self.render(event_loop);
                match self.window.as_ref() {
                    Some(it) => it.request_redraw(),
                    None => panic!("window does not seem to exist."),
                }
            }
            _ => (),
        }
    }
}

impl Emulator {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let is_on = self.core.display[i];
                let color = if is_on { 0xFF } else { 0x00 };
                pixel[0] = color; // R
                pixel[1] = color; // G
                pixel[2] = color; // B
                pixel[3] = 0xFF; // A (Always 255 for opaque)
            }
            if let Err(e) = pixels.render() {
                println!("Render error: {}", e);
                event_loop.exit();
            }
        }
    }
}
