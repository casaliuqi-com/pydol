use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{EventLoopWindowTarget, ControlFlow, EventLoop}
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

mod window;

pub fn run(initial_url: impl Into<Option<String>>) {
    let event_loop = EventLoop::new();
    window::add_default(&event_loop);
    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, window_id } => {
                match event {
                    WindowEvent::CloseRequested => {
                        window::remove(&window_id);
                        if window::dict_is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    _ => {}
                }
            }
            _=> {}
        }
    });
}




