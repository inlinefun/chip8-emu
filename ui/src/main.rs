mod emulator;
mod util;

use crate::emulator::Emulator;

fn main() {
    let mut emulator = Emulator::default();
    emulator.core.load_rom("corax.ch8");
    let event_loop = util::init_event_loop();
    match event_loop.run_app(&mut emulator) {
        Ok(_) => (),
        Err(e) => panic!("failed to run the 'winit' app. \n{}", e),
    }
}
