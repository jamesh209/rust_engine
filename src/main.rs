mod engine_window;
mod engine_loop;

use std::env;


fn main() {    
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut engine_loop = engine_loop::EngineLoop::new();   

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(engine_loop.create_new_viewport());
    runtime.block_on(engine_loop.start());
}