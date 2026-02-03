use winit::event_loop::{ControlFlow, EventLoop};

pub(crate) fn init_event_loop() -> EventLoop<()> {
    let it = match EventLoop::new() {
        Ok(it) => it,
        Err(e) => panic!("failed to initalize an event loop for 'winit'. \n{}", e),
    };
    it.set_control_flow(ControlFlow::Poll);
    return it;
}
