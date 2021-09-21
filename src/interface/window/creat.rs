use std::path::Path;
use image;
use winit::{
    window::{self, Icon, WindowBuilder},
    event_loop::EventLoopWindowTarget
};

use super::core;

#[inline]
pub fn creat_window(
    event_loop: &EventLoopWindowTarget<()>,
    window_type: Option<core::WindowType>
) -> window::Window {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/logo.png");
    let icon = load_icon(Path::new(path));
    let window_type = window_type.unwrap_or(core::WindowType::default());
    let window = WindowBuilder::new()
        .with_title("Pydol")
        .with_window_icon(Some(icon))
        .build(event_loop)
        .unwrap();
    window.set_decorations(false);
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
