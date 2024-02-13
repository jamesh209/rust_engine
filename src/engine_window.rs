use winit::{
    event::{WindowEvent},
    event_loop::{EventLoop},
    window::{Window, WindowBuilder}
};
pub trait WindowContenProvider {
    fn handle_inputs(&mut self, event : &WindowEvent);

    fn redraw(&mut self);
}


pub struct EngineViewport{
    window: Window,
}

impl WindowContenProvider for EngineViewport {
    fn handle_inputs(&mut self, event : &WindowEvent) {
        println!("{event:?}");
    }

    fn redraw(&mut self) {
        
    }
}

impl EngineViewport {
    pub fn new(event_loop: &EventLoop<()>, window_builder: WindowBuilder) -> EngineViewport {

        let window = window_builder.build(&event_loop).unwrap();

        Self{
            window
        }
    }

    pub fn new_as_provider(event_loop: &EventLoop<()>, window_builder: WindowBuilder) -> Box<dyn WindowContenProvider> {
        return Box::new(EngineViewport::new(event_loop, window_builder));
    }
}