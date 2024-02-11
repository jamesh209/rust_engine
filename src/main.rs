mod engine_test;
use engine_test::run;

use std::env;


fn main() {    
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(run());
}
