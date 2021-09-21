use std::collections::HashMap;
use std::cell::RefCell;

use winit::{
    window::{self, WindowId},
    event_loop::EventLoopWindowTarget
};

mod core;
mod creat;

thread_local! {
    static DICT_WINDOW: RefCell<HashMap<WindowId, core::Window>> = RefCell::new(HashMap::new());
}

pub fn dict_is_empty() -> bool {
    DICT_WINDOW.with(|dict_win|{
        dict_win.borrow().is_empty()
    })
}

pub fn add_default(
    event_loop: &EventLoopWindowTarget<()>
) {
    let window = creat::creat_window(event_loop, None);
    DICT_WINDOW.with(|dict_win|{
        dict_win.borrow_mut().insert(window.id(), core::Window::new(window, core::WindowType::default()));
    });
}

pub fn remove(id: &WindowId){
    DICT_WINDOW.with(|dict_win|{
        dict_win.borrow_mut().remove(id);
    });
}

#[inline]
pub fn with_id<F, R>(id: &WindowId, f: F) -> Option<R>
where
    F: FnOnce(&mut core::Window) -> R {
    DICT_WINDOW.with(|dict_win|{
        if dict_win.borrow().contains_key(id) {
            let result = f(dict_win.borrow_mut().get_mut(id).unwrap());
            Some(result)
        } else {
            None
        }
    })
}

#[inline]
pub fn is_maximized(id: &WindowId) -> bool {
    if let Some(result) = with_id(id, |this|{
        this.get_context().is_maximized()
    }) {
        return result;
    }
    false
}
