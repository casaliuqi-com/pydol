use std::collections::HashMap;
use std::path::Path;
use std::cell::RefCell;
use image;

#[allow(unused_imports)]
use winit::{
    window::{Icon, Window, WindowId, WindowBuilder},
    event_loop::EventLoopWindowTarget
};

enum WindowType {
    Default,
    Debugger
}

thread_local! {
    static DICT_WINDOW: RefCell<HashMap<WindowId, (Window, WindowType)>> = RefCell::new(HashMap::new());
}

pub fn dict_is_empty() -> bool {
    let mut result = false;
    DICT_WINDOW.with(|dict_win|{
        result = dict_win.borrow().is_empty();
    });
    result
}

pub fn add_default(
    event_loop: &EventLoopWindowTarget<()>
) {
    let window = creat_window(event_loop, None);
    DICT_WINDOW.with(|dict_win|{
        dict_win.borrow_mut().insert(window.id(), (window, WindowType::Default));
    });
}

pub fn remove(id: &WindowId){
    DICT_WINDOW.with(|dict_win|{
        dict_win.borrow_mut().remove(id);
    });
}

#[inline]
fn creat_window(
    event_loop: &EventLoopWindowTarget<()>,
    window_type: Option<WindowType>
) -> Window{
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/logo.png");
    let icon = load_icon(Path::new(path));
    let window_type = window_type.unwrap_or(WindowType::Default);
    let window = WindowBuilder::new()
        .with_title("Pydol")
        .with_window_icon(Some(icon))
        .build(event_loop)
        .unwrap();
    //window.set_decorations(false);
    window.set_visible(true);
    window.request_redraw();
    window
}

#[inline]
fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
