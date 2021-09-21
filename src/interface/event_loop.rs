use std::cell::RefCell;

use winit::{
    dpi::PhysicalPosition,
    window::{Window, WindowId},
    event::{ElementState, Event, WindowEvent, MouseButton},
    event_loop::{EventLoopWindowTarget, ControlFlow, EventLoop}
};
// use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use super::window;

thread_local! {
    static WINDOW_CURSOR: RefCell<Option<(WindowId, i32, i32, bool, u8)>> = RefCell::new(None);
}

#[inline]
fn set_window_cursor(id: WindowId, position: PhysicalPosition<f64>){
    let position: (i32, i32) = position.into();
    match get_window_cursor(){
        Some(temp) => WINDOW_CURSOR.with(|cursor|{
            *cursor.borrow_mut() = Some((id, position.0, position.1, temp.3, temp.4));
        }),
        None => WINDOW_CURSOR.with(|cursor|{
            *cursor.borrow_mut() = Some((id, position.0, position.1, false, u8::MAX));
        })
    }
    
}

#[inline]
fn get_window_cursor() -> Option<(WindowId, i32, i32, bool, u8)> {
    WINDOW_CURSOR.with(|cursor|{
        *cursor.borrow()
    })
}

#[inline]
fn drag(id: WindowId, position: PhysicalPosition<f64>){
    let position: (i32, i32) = position.into();
    match get_window_cursor() {
        Some(temp) => {
            if temp.0 != id 
            || !temp.3 
            || temp.4 != 0 
            || window::is_maximized(&id){
                return;
            }
            let (dx, dy) = (position.0 - temp.1, position.1 - temp.2);
            window::with_id(&id, |this|{
                match this.get_context().outer_position() {
                    Ok(window_position) => {
                        let window_position: (i32, i32) = window_position.into();
                        let (x, y) = (window_position.0 + dx, window_position.1 + dy);
                        this.get_context().set_outer_position(PhysicalPosition::new(x, y));
                    }
                    _ => {}
                }
            });
        }
        _=> {}
    }
}

fn set_mousestate(button: MouseButton, state: ElementState) {
    let state = match state {
        ElementState::Pressed => true,
        _ => false
    };
    let button = match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        _ => u8::MAX
    };
    match get_window_cursor(){
        Some(temp) => WINDOW_CURSOR.with(|cursor|{
            *cursor.borrow_mut() = Some((temp.0, temp.1, temp.2, state, button));
        }),
        None => {}
    }
}

#[inline]
fn clean_window_cursor(){
    WINDOW_CURSOR.with(|cursor|{
        *cursor.borrow_mut() = None;
    });
}

#[inline]
fn equal_window_cursor(window_id: WindowId) -> bool {
    WINDOW_CURSOR.with(|cursor|{
        if let Some(cursor) = *cursor.borrow() {
            return cursor.0 == window_id;
        }
        false
    })
}

fn has_window_cursor() -> bool {
    WINDOW_CURSOR.with(|cursor|{
        match *cursor.borrow() {
            None => false,
            _ => true
        }
    })
}

#[inline]
fn draw_core(id: &WindowId, frame: &mut [u8], width: u32, height: u32){
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        let rgba = if y > 36 * 2 {
            [255, 255, 255, 255]
        } else {
            [225, 225, 225, 255]
        };
        pixel.copy_from_slice(&rgba);
    }
}

fn draw(id: &WindowId) -> bool {
    if let Some(result) = window::with_id(id, |this|{
        let window_size = this.get_inner_size();
        let frame = this.get_frame();
        draw_core(id, frame.get_frame(), window_size.width, window_size.height);
        !frame.render().is_err()
    }) {
        return result;
    }
    false
}

pub fn run(initial_url: impl Into<Option<String>>) {
    let initial_url = initial_url.into();
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
                    WindowEvent::Resized(size) => {
                        window::with_id(&window_id, |this|{
                            this.reset_frame(size);
                            this.get_context().request_redraw();
                        });
                    }
                    WindowEvent::CursorMoved{position, ..} => {
                        drag(window_id, position);
                        set_window_cursor(window_id, position);
                    }
                    WindowEvent::MouseInput{button, state, ..} => {
                        set_mousestate(button, state);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) => {
                if !draw(&window_id) {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _=> {}
        }
    });
}
