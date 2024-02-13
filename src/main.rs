mod engine_window;
mod engine_loop;

use std::env;
use winit::window::WindowBuilder;


fn main() {    
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut engine_loop = engine_loop::EngineLoop::new();
    
    let viewport_window_builder = WindowBuilder::new();
    
    engine_loop.add_new_window(|event_loop| engine_window::EngineViewport::new_as_provider(event_loop, viewport_window_builder));
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(engine_loop.start());
}
