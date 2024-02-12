mod engine_loop;

use std::env;


fn main() {    
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let engine_loop = engine_loop::EngineLoop::new();

    runtime.block_on(engine_loop.start());
}
