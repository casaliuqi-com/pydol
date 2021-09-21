use winit::{
    window,
    dpi::PhysicalSize
};
use pixels::{Pixels, SurfaceTexture};

#[derive(Copy, Clone)]
pub enum WindowType {
    Default,
    Debugger
}

impl Default for WindowType {
    fn default() -> Self {
        WindowType::Default
    }
}

pub struct Window {
    context: window::Window, 
    frame: Pixels,
    r#type: WindowType,
    size: PhysicalSize<u32>
}

impl Window {
    pub fn new(context: window::Window, r#type: WindowType) -> Self {
        let window_size = context.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &context);
        let frame = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
        Window{context, frame, r#type, size: window_size}
    }
    
    pub fn get_context(&self) -> &window::Window {
        &self.context
    } 
    
    pub fn get_frame(&mut self) -> &mut Pixels {
        &mut self.frame
    }
    
    pub fn reset_frame(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.frame.resize_surface(size.width, size.height);
        self.frame.resize_buffer(size.width, size.height);
    }
    
    pub fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.size
    }
}
