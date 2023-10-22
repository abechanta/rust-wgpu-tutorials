use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey},
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let _ = event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            window_id, ref event
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested |
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::Escape), ..
                },
                ..
            } => {
                control_flow.exit()
            },
            _ => {},
        },
        _ => {},
    });
}